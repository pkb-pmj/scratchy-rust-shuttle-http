pub mod project;
pub mod studio;
pub mod user;

use async_trait::async_trait;
use reqwest::Client;

pub use project::Project;
pub use studio::Comment;
pub use user::User;

use super::{GetUrl, ScratchAPIError};

#[async_trait]
pub trait ScratchAPIClient {
    type Error;

    async fn get_scratch_api_project(
        &self,
        project_id: i64,
    ) -> Result<Option<Project>, Self::Error>;

    async fn get_scratch_api_studio_comments(
        &self,
        studio_id: i64,
    ) -> Result<Option<Vec<Comment>>, Self::Error>;

    async fn get_scratch_api_user(&self, username: &str) -> Result<Option<User>, Self::Error>;
}

#[async_trait]
impl ScratchAPIClient for Client {
    type Error = ScratchAPIError;

    async fn get_scratch_api_project(
        &self,
        project_id: i64,
    ) -> Result<Option<Project>, Self::Error> {
        self.get_url_optional(format!("https://api.scratch.mit.edu/projects/{project_id}"))
            .await
    }

    async fn get_scratch_api_studio_comments(
        &self,
        studio_id: i64,
    ) -> Result<Option<Vec<Comment>>, Self::Error> {
        self.get_url_optional(format!(
            "https://api.scratch.mit.edu/studios/{studio_id}/comments"
        ))
        .await
    }

    async fn get_scratch_api_user(&self, username: &str) -> Result<Option<User>, Self::Error> {
        self.get_url_optional(format!("https://api.scratch.mit.edu/users/{username}"))
            .await
    }
}
