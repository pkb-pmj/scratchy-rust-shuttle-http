use async_trait::async_trait;
use sqlx::{Executor, Postgres};
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
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    /// Just to make sure it compiles with all the lifetimes
    #[sqlx::test]
    async fn lifetime_compile_test(pool: PgPool) {
        pool.get_scratch_account("username".into()).await.unwrap();

        let mut tx = pool.begin().await.unwrap();

        tx.get_scratch_account("username".into()).await.unwrap();

        pool.get_scratch_account("username".into()).await.unwrap();

        tx.get_scratch_account("username".into()).await.unwrap();

        tx.commit().await.unwrap();
    }
}
