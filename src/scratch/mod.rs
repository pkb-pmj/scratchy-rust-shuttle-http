use reqwest::{Client, Error, StatusCode};
use serde::Deserialize;
use thiserror::Error;

pub mod api;
pub mod db;
pub mod site;

pub const STUDIO_ID: i64 = 29137750;
pub const STUDIO_URL: &str = "https://scratch.mit.edu/studios/29137750/comments";

pub trait Url {
    type UrlArgs;

    fn url(args: Self::UrlArgs) -> String;
}

pub trait Requestable: Url + for<'de> Deserialize<'de> {}

impl<T: Url + for<'de> Deserialize<'de>> Requestable for T {}

#[derive(Clone)]
pub struct ScratchClient(Client);

impl ScratchClient {
    pub fn new() -> Self {
        Self(Client::new())
    }

    pub async fn get<T: Requestable>(&self, input: T::UrlArgs) -> Result<T, ScratchAPIError> {
        Ok(self
            .0
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
