#[cfg(test)]
mod tests;

use async_trait::async_trait;
use sqlx::{Executor, PgPool, Postgres};
use time::OffsetDateTime;
use twilight_model::id::{marker::UserMarker, Id};

use crate::linked_roles::{RoleConnectionData, Token};

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

    async fn transfer_linked_scratch_accounts(
        self,
        from: Id<UserMarker>,
        to: Id<UserMarker>,
    ) -> Result<Vec<String>, Self::Error>;

    async fn get_token(self, id: Id<UserMarker>) -> Result<Option<Token>, Self::Error>;

    async fn write_token(self, id: Id<UserMarker>, token: Token) -> Result<Token, Self::Error>;

    async fn delete_token(self, id: Id<UserMarker>) -> Result<Token, Self::Error>;

    async fn get_oldest_metadata(
        self,
    ) -> Result<Option<(Id<UserMarker>, OffsetDateTime)>, Self::Error>;

    async fn get_metadata(
        self,
        id: Id<UserMarker>,
    ) -> Result<Option<RoleConnectionData>, Self::Error>;

    async fn write_metadata(
        self,
        id: Id<UserMarker>,
        data: &RoleConnectionData,
    ) -> Result<RoleConnectionData, Self::Error>;
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

    async fn transfer_linked_scratch_accounts(
        self,
        from: Id<UserMarker>,
        to: Id<UserMarker>,
    ) -> Result<Vec<String>, Self::Error> {
        sqlx::query!(
            r#"
                UPDATE scratch_accounts
                SET id = $2
                WHERE id = $1
                RETURNING username
            "#,
            from.to_string(),
            to.to_string(),
        )
        .map(|row| row.username)
        .fetch_all(self)
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

    async fn delete_token(self, id: Id<UserMarker>) -> Result<Token, Self::Error> {
        sqlx::query_as!(
            Token,
            r#"
                DELETE FROM tokens
                WHERE id = $1
                RETURNING access_token, refresh_token, expires_at
            "#,
            id.to_string(),
        )
        .fetch_one(self)
        .await
    }

    async fn get_oldest_metadata(
        self,
    ) -> Result<Option<(Id<UserMarker>, OffsetDateTime)>, Self::Error> {
        sqlx::query!(
            r#"
                SELECT id, updated_at
                FROM metadata
                ORDER BY updated_at ASC
                LIMIT 1
            "#
        )
        .map(|row| (row.id.parse().unwrap(), row.updated_at))
        .fetch_optional(self)
        .await
    }

    async fn get_metadata(
        self,
        id: Id<UserMarker>,
    ) -> Result<Option<RoleConnectionData>, Self::Error> {
        sqlx::query_as!(
            RoleConnectionData,
            r#"
                SELECT scratcher, followers, joined
                FROM metadata
                WHERE id = $1
            "#,
            id.to_string(),
        )
        .fetch_optional(self)
        .await
    }

    async fn write_metadata(
        self,
        id: Id<UserMarker>,
        data: &RoleConnectionData,
    ) -> Result<RoleConnectionData, Self::Error> {
        sqlx::query_as!(
            RoleConnectionData,
            r#"
                UPDATE metadata SET
                    scratcher = $2,
                    followers = $3,
                    joined = $4,
                    updated_at = 'now'
                WHERE id = $1
                RETURNING scratcher, followers, joined
            "#,
            id.to_string(),
            data.scratcher,
            data.followers,
            data.joined,
        )
        .fetch_one(self)
        .await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkError {
    AlreadyLinkedToYou,
    AlreadyLinkedToOther(Id<UserMarker>),
}

pub async fn link_account(
    pool: &PgPool,
    username: String,
    id: Id<UserMarker>,
) -> Result<Result<(), LinkError>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    if let Some(already_linked) = tx.get_scratch_account(username.to_owned()).await? {
        if already_linked.id == id {
            return Ok(Err(LinkError::AlreadyLinkedToYou));
        } else {
            return Ok(Err(LinkError::AlreadyLinkedToOther(already_linked.id)));
        }
    }

    if tx.get_discord_account(id).await?.is_none() {
        tx.create_discord_account(id).await?;
    }

    tx.create_linked_scratch_account(username, id).await?;

    tx.commit().await?;

    Ok(Ok(()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferError {
    AlreadyLinkedToYou,
    NotLinked,
}

pub async fn transfer_linked_accounts(
    pool: &PgPool,
    username: String,
    id: Id<UserMarker>,
) -> Result<Result<(Id<UserMarker>, Vec<String>), TransferError>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let already_linked = match tx.get_scratch_account(username.to_owned()).await? {
        Some(already_linked) => {
            if already_linked.id == id {
                return Ok(Err(TransferError::AlreadyLinkedToYou));
            } else {
                already_linked
            }
        }
        None => return Ok(Err(TransferError::NotLinked)),
    };

    if tx.get_discord_account(id).await?.is_none() {
        tx.create_discord_account(id).await?;
    }

    let transferred = tx
        .transfer_linked_scratch_accounts(already_linked.id, id)
        .await?;

    tx.commit().await?;

    Ok(Ok((already_linked.id, transferred)))
}
