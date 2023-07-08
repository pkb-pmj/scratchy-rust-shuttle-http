use async_trait::async_trait;
use reqwest::{Client, Error, StatusCode};
use serde::Deserialize;
use thiserror::Error;

pub mod api;
pub mod db;
pub mod site;

pub const STUDIO_ID: i64 = 29137750;
pub const STUDIO_URL: &str = "https://scratch.mit.edu/studios/29137750/comments";

pub trait Url {
    type UrlArgs: Send;

    fn url(args: Self::UrlArgs) -> String;
}

pub trait Requestable: Url + for<'de> Deserialize<'de> {}

impl<T: Url + for<'de> Deserialize<'de>> Requestable for T {}

#[async_trait]
pub trait ScratchClient {
    type Error;

    async fn get_scratch<T: Requestable>(&self, input: T::UrlArgs) -> Result<T, Self::Error>;
}

#[async_trait]
impl ScratchClient for Client {
    type Error = ScratchAPIError;

    async fn get_scratch<T: Requestable>(&self, input: T::UrlArgs) -> Result<T, Self::Error> {
        Ok(self
            .get(T::url(input))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

#[derive(Debug, Error)]
pub enum ScratchAPIError {
    #[error("Not found")]
    NotFound,
    #[error("Server error")]
    ServerError,
    #[error("Other")]
    Other(Error),
}

impl From<Error> for ScratchAPIError {
    fn from(value: Error) -> Self {
        match value.status() {
            Some(StatusCode::NOT_FOUND) => ScratchAPIError::NotFound,
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
