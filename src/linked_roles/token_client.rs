use async_trait::async_trait;
use oauth2::{
    basic::BasicErrorResponseType, reqwest::async_http_client, RequestTokenError, TokenResponse,
};
use time::OffsetDateTime;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{database::Database, state::AppState};

use super::client::OAuthError;
use super::{OAuthToken, Token};

#[async_trait]
pub trait TokenClient {
    type Error;

    async fn get_active_token(&self, id: Id<UserMarker>) -> Result<Token, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("user not authorized")]
    UserNotAuthorized,
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    OAuthError(#[from] OAuthError),
}

#[async_trait]
impl TokenClient for AppState {
    type Error = TokenError;

    async fn get_active_token(&self, id: Id<UserMarker>) -> Result<Token, Self::Error> {
        let mut tx = self.pool.begin().await?;

        let mut token = tx
            .get_token(id)
            .await?
            .ok_or(Self::Error::UserNotAuthorized)?;

        if token.expires_at < OffsetDateTime::now_utc() {
            let oauth_token: OAuthToken = token.into();

            let new_token = match self
                .oauth_client
                // Safe to unwrap because we assume Discord returns all the necessary fields
                .exchange_refresh_token(&oauth_token.refresh_token().unwrap())
                .request_async(async_http_client)
                .await
            {
                Ok(value) => value,
                Err(RequestTokenError::ServerResponse(err))
                    if matches!(err.error(), BasicErrorResponseType::InvalidGrant) =>
                {
                    tx.delete_token(id).await?;
                    Err(TokenError::UserNotAuthorized)?
                }
                Err(err) => Err(err)?,
            }
            .try_into()
            .unwrap();

            token = tx.write_token(id, new_token).await?;
        }

        tx.commit().await?;

        Ok(token)
    }
}
