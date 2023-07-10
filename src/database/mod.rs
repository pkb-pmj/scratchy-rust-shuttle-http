#[cfg(test)]
mod tests;

use async_trait::async_trait;
use sqlx::{Executor, PgPool, Postgres};
use twilight_model::id::{marker::UserMarker, Id};

use crate::linked_roles::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscordAccount {
    pub id: Id<UserMarker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    async fn get_token(self, id: Id<UserMarker>) -> Result<Option<Token>, Self::Error>;

    async fn write_token(self, id: Id<UserMarker>, token: Token) -> Result<Token, Self::Error>;
}

// Not sure how this works, but it works
// https://users.rust-lang.org/t/removing-the-lifetime-from-this-function-signature/92088/5
#[async_trait]
impl<'a, T> Database for T
where
    T: Executor<'a, Database = Postgres>,
{
    type Error = sqlx::Error;

    async fn get_discord_account(
        self,
        id: Id<UserMarker>,
    ) -> Result<Option<DiscordAccount>, Self::Error> {
        sqlx::query!(
            r#"
                SELECT *
                FROM discord_accounts
                WHERE id = $1
            "#,
            id.to_string(),
        )
        .map(|user| DiscordAccount {
            id: user.id.parse().unwrap(),
        })
        .fetch_optional(self)
        .await
    }

    async fn create_discord_account(
        self,
        id: Id<UserMarker>,
    ) -> Result<DiscordAccount, Self::Error> {
        sqlx::query!(
            r#"
                INSERT INTO discord_accounts (id)
                VALUES ($1)
                RETURNING *
            "#,
            id.to_string(),
        )
        .map(|user| DiscordAccount {
            id: user.id.parse().unwrap(),
        })
        .fetch_one(self)
        .await
    }

    async fn get_scratch_account(
        self,
        username: String,
    ) -> Result<Option<ScratchAccount>, Self::Error> {
        sqlx::query!(
            r#"
                SELECT *
                FROM scratch_accounts
                WHERE lower(username) = lower($1)
            "#,
            username,
        )
        .map(|user| ScratchAccount {
            username: user.username,
            id: user.id.parse().unwrap(),
        })
        .fetch_optional(self)
        .await
    }

    async fn get_linked_scratch_accounts(
        self,
        id: Id<UserMarker>,
    ) -> Result<Vec<ScratchAccount>, Self::Error> {
        sqlx::query!(
            r#"
                SELECT *
                FROM scratch_accounts
                WHERE id = $1
            "#,
            id.to_string(),
        )
        .map(|user| ScratchAccount {
            username: user.username,
            id: user.id.parse().unwrap(),
        })
        .fetch_all(self)
        .await
    }

    async fn create_linked_scratch_account(
        self,
        username: String,
        id: Id<UserMarker>,
    ) -> Result<ScratchAccount, Self::Error> {
        sqlx::query!(
            r#"
                INSERT INTO scratch_accounts (username, id)
                VALUES ($1, $2)
                RETURNING *
            "#,
            username.to_string(),
            id.to_string(),
        )
        .map(|row| ScratchAccount {
            username: row.username,
            id: row.id.parse().unwrap(),
        })
        .fetch_one(self)
        .await
    }

    async fn get_token(self, id: Id<UserMarker>) -> Result<Option<Token>, Self::Error> {
        Ok(sqlx::query_as!(
            Token,
            r#"
                SELECT access_token, refresh_token, expires_at
                FROM tokens
                WHERE id = $1
            "#,
            id.to_string(),
        )
        .fetch_optional(self)
        .await?)
    }

    async fn write_token(self, id: Id<UserMarker>, token: Token) -> Result<Token, Self::Error> {
        Ok(sqlx::query_as!(
            Token,
            r#"
                INSERT INTO tokens (id, access_token, refresh_token, expires_at)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (id) DO UPDATE SET
                    id = EXCLUDED.id,
                    access_token = EXCLUDED.access_token,
                    refresh_token = EXCLUDED.refresh_token,
                    expires_at = EXCLUDED.expires_at
                RETURNING access_token, refresh_token, expires_at
            "#,
            id.to_string(),
            token.access_token,
            token.refresh_token,
            token.expires_at,
        )
        .fetch_one(self)
        .await?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkResult {
    AlreadyLinkedToYou,
    AlreadyLinkedToOther(Id<UserMarker>),
    SuccessfullyLinked,
}

pub async fn link_account(
    pool: &PgPool,
    username: String,
    id: Id<UserMarker>,
) -> Result<LinkResult, sqlx::Error> {
    let mut tx = pool.begin().await?;

    if let Some(already_linked) = tx.get_scratch_account(username.to_owned()).await? {
        if already_linked.id == id {
            return Ok(LinkResult::AlreadyLinkedToYou);
        } else {
            return Ok(LinkResult::AlreadyLinkedToOther(already_linked.id));
        }
    }

    if tx.get_discord_account(id).await?.is_none() {
        tx.create_discord_account(id).await?;
    }

    tx.create_linked_scratch_account(username, id).await?;

    tx.commit().await?;

    Ok(LinkResult::SuccessfullyLinked)
}
