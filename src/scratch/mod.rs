use async_trait::async_trait;
use reqwest::{Client, Error, IntoUrl, StatusCode};
use serde::Deserialize;
use thiserror::Error;

pub mod api;
pub mod db;
pub mod site;

pub const STUDIO_ID: i64 = 29137750;
pub const STUDIO_URL: &str = "https://scratch.mit.edu/studios/29137750/comments";

#[async_trait]
trait GetUrl {
    type Error;

    async fn get_url<T: for<'de> Deserialize<'de>>(
        &self,
        url: impl IntoUrl + Send,
    ) -> Result<T, Self::Error>;

    async fn get_url_optional<T: for<'de> Deserialize<'de>>(
        &self,
        url: impl IntoUrl + Send,
    ) -> Result<Option<T>, Self::Error>;
}

#[async_trait]
impl GetUrl for Client {
    type Error = ScratchAPIError;

    async fn get_url<T: for<'de> Deserialize<'de>>(
        &self,
        url: impl IntoUrl + Send,
    ) -> Result<T, Self::Error> {
        Ok(self
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    async fn get_url_optional<T: for<'de> Deserialize<'de>>(
        &self,
        url: impl IntoUrl + Send,
    ) -> Result<Option<T>, Self::Error> {
        Ok(match self.get(url).send().await?.error_for_status() {
            Ok(res) => Some(res.json().await?),
            Err(err) if err.status() == Some(StatusCode::NOT_FOUND) => None,
            Err(err) => Err(err)?,
        })
    }
}

#[derive(Debug, Error)]
pub enum ScratchAPIError {
    #[error("Server error")]
    ServerError,
    #[error("Other")]
    Other(Error),
}

impl From<Error> for ScratchAPIError {
    fn from(value: Error) -> Self {
        match value.status() {
            Some(status) => {
                if status.is_server_error() {
                    ScratchAPIError::ServerError
                } else {
                    ScratchAPIError::Other(value)
                }
            }
            _ => ScratchAPIError::Other(value),
        }
    }
}
