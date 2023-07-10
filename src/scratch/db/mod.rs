pub mod project;
pub mod user;

use async_trait::async_trait;
use reqwest::Client;

pub use project::Project;
pub use user::User;

use super::{GetUrl, ScratchAPIError};

#[async_trait]
pub trait ScratchDBClient {
    type Error;

    async fn get_scratch_db_user(&self, username: &str) -> Result<User, Self::Error>;

    async fn get_scratch_db_project(&self, id: i64) -> Result<Project, Self::Error>;
}

#[async_trait]
impl ScratchDBClient for Client {
    type Error = ScratchAPIError;

    async fn get_scratch_db_user(&self, username: &str) -> Result<User, Self::Error> {
        self.get_url(format!(
            "https://scratchdb.lefty.one/v3/user/info/{username}"
        ))
        .await
    }

    async fn get_scratch_db_project(&self, id: i64) -> Result<Project, Self::Error> {
        self.get_url(format!("https://scratchdb.lefty.one/v3/project/info/{id}"))
            .await
    }
}
