use reqwest::{Client, Error, StatusCode};
use serde::Deserialize;
use thiserror::Error;

pub mod api;
pub mod db;

#[derive(Debug, Error)]
pub enum ScratchAPIError {
    #[error("Not found")]
    NotFound,
    #[error("Other")]
    Other(Error),
}

impl From<Error> for ScratchAPIError {
    fn from(value: Error) -> Self {
        match value.status() {
            Some(StatusCode::NOT_FOUND) => ScratchAPIError::NotFound,
            _ => ScratchAPIError::Other(value),
        }
    }
}

pub async fn get<T: for<'de> Deserialize<'de>>(
    client: Client,
    url: String,
) -> Result<T, ScratchAPIError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}
