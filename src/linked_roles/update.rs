use async_trait::async_trait;
use oauth2::{reqwest::async_http_client, TokenResponse};
use reqwest::Client;
use time::OffsetDateTime;
use tokio::task::JoinSet;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    database::{Database, ScratchAccount},
    scratch::{
        db::{self, user::Status, ScratchDBClient},
        ScratchAPIError,
    },
    state::AppState,
};

use super::{
    client::RoleConnectionClient, metadata::RoleConnectionData, model::RoleConnection, OAuthToken,
};

#[async_trait]
pub trait RoleConnectionUpdater {
    type Error;

    async fn update_role_connection(
        &self,
        id: Id<UserMarker>,
    ) -> Result<RoleConnection<RoleConnectionData>, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum RoleConnectionUpdateError {
    #[error("user not authorized")]
    UserNotAuthorized,
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    OAuthError(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),
    #[error(transparent)]
    ScratchAPIError(#[from] ScratchAPIError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

#[async_trait]
impl RoleConnectionUpdater for AppState {
    type Error = RoleConnectionUpdateError;

    async fn update_role_connection(
        &self,
        id: Id<UserMarker>,
    ) -> Result<RoleConnection<RoleConnectionData>, Self::Error> {
        let mut tx = self.pool.begin().await?;

        let mut token = tx
            .get_token(id)
            .await?
            .ok_or(Self::Error::UserNotAuthorized)?;

        if token.expires_at < OffsetDateTime::now_utc() {
            let oauth_token: OAuthToken = token.into();

            // Safe to unwrap because we assume Discord returns all the necessary fields
            let new_token = self
                .oauth_client
                .exchange_refresh_token(&oauth_token.refresh_token().unwrap())
                .request_async(async_http_client)
                .await?
                .try_into()
                .unwrap();

            token = tx.write_token(id, new_token).await?;
        }

        let linked_accounts = tx.get_linked_scratch_accounts(id).await?;

        let accounts = fetch_scratch_data(linked_accounts, &self.reqwest_client).await?;

        let role_connection = find_metadata_values(accounts);

        self.reqwest_client
            .put_role_connection(
                &self.config.client_id,
                &token.access_token,
                &role_connection,
            )
            .await?;

        Ok(role_connection)
    }
}

async fn fetch_scratch_data(
    linked_accounts: Vec<ScratchAccount>,
    client: &Client,
) -> Result<Vec<db::User>, ScratchAPIError> {
    let mut db_accounts = Vec::with_capacity(linked_accounts.len());

    let mut set = JoinSet::new();

    for account in linked_accounts {
        let client = client.clone();
        set.spawn(async move { client.get_scratch_db_user(&account.username).await });
    }

    while let Some(result) = set.join_next().await {
        db_accounts.push(result.unwrap()?);
    }

    Ok(db_accounts)
}

/// Finds if any account has Scratcher status, the highest number of followers and the oldest account.
/// Username is taken from the account with the most followers.
///
/// # Panics
///
/// Panics if `accounts` is empty.
fn find_metadata_values(accounts: Vec<db::User>) -> RoleConnection<RoleConnectionData> {
    let scratcher = accounts
        .iter()
        .any(|account| account.status == Some(Status::Scratcher));

    let joined = accounts.iter().map(|account| account.joined).min().unwrap();

    let max_followers = accounts
        .into_iter()
        .max_by_key(|account| {
            account
                .statistics
                .as_ref()
                .map(|statistics| statistics.followers)
                .unwrap_or(0)
        })
        .unwrap();

    RoleConnection {
        platform_name: Some("Scratch".into()),
        platform_username: Some(max_followers.username),
        metadata: RoleConnectionData {
            scratcher,
            followers: max_followers
                .statistics
                .map(|statistics| statistics.followers)
                .unwrap_or(0),
            joined,
        },
    }
}
