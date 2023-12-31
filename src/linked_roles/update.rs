use async_trait::async_trait;
use reqwest::Client;
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
    client::{OAuthError, RoleConnectionClient},
    metadata::RoleConnectionData,
    model::RoleConnection,
    token_client::{TokenClient, TokenError},
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
    #[error(transparent)]
    TokenError(#[from] TokenError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    OAuthError(#[from] OAuthError),
    #[error(transparent)]
    ScratchAPIError(#[from] ScratchAPIError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("no accounts found in ScratchDB for user {0}")]
    NoAccountsFound(Id<UserMarker>),
}

#[async_trait]
impl RoleConnectionUpdater for AppState {
    type Error = RoleConnectionUpdateError;

    async fn update_role_connection(
        &self,
        id: Id<UserMarker>,
    ) -> Result<RoleConnection<RoleConnectionData>, Self::Error> {
        let token = self.get_active_token(id).await?;

        let mut tx = self.pool.begin().await?;

        let linked_accounts = tx.get_linked_scratch_accounts(id).await?;

        let accounts = fetch_scratch_data(linked_accounts, &self.reqwest_client).await?;
        if accounts.len() == 0 {
            return Err(RoleConnectionUpdateError::NoAccountsFound(id));
        }

        let role_connection = find_metadata_values(accounts);

        let old_data = tx.get_metadata(id).await?;

        // Write even if unchanged to update `updated_at`
        tx.write_metadata(id, &role_connection.metadata).await?;

        tx.commit().await?;

        // Only update if the metadata has changed or was `None`
        if old_data.as_ref() != Some(&role_connection.metadata) {
            self.reqwest_client
                .put_role_connection(
                    &self.config.client_id,
                    &token.access_token,
                    &role_connection,
                )
                .await?;
        }

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
        // Skip accounts which aren't in ScratchDB
        if let Some(user) = result.unwrap()? {
            db_accounts.push(user);
        }
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
