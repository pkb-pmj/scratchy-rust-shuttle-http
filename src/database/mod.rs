use async_trait::async_trait;
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Debug, Clone)]
pub struct ScratchUser {
    pub username: String,
    pub id: Id<UserMarker>,
}

#[derive(Debug, Clone)]
pub struct DiscordAccount {
    pub id: Id<UserMarker>,
}

#[derive(Debug, Clone)]
pub struct ScratchAccount {
    pub username: String,
    pub id: Id<UserMarker>,
}

#[async_trait]
pub trait Database {
    type Error;

    async fn get_discord_account(
        self,
        id: Id<UserMarker>,
    ) -> Result<Option<DiscordAccount>, Self::Error>;

    async fn create_discord_account(
        self,
        id: Id<UserMarker>,
    ) -> Result<DiscordAccount, Self::Error>;

    async fn get_scratch_account(
        self,
        username: String,
    ) -> Result<Option<ScratchAccount>, Self::Error>;

    async fn get_linked_scratch_accounts(
        self,
        id: Id<UserMarker>,
    ) -> Result<Vec<ScratchAccount>, Self::Error>;

    async fn create_linked_scratch_account(
        self,
        username: String,
        id: Id<UserMarker>,
    ) -> Result<ScratchAccount, Self::Error>;
}
