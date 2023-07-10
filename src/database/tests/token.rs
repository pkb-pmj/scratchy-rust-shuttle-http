use sqlx::PgPool;
use time::{macros::datetime, Duration, OffsetDateTime};

use crate::{database::Database, linked_roles::Token};

#[sqlx::test(fixtures("linked_accounts", "tokens"))]
async fn read_existing_token(pool: PgPool) {
    let id = "755497867606622450".parse().unwrap();

    let expected = Token {
        access_token: "access_token".into(),
        refresh_token: "refresh_token".into(),
        expires_at: datetime!(2023-07-10 12:00:00 UTC),
    };

    let actual = pool.get_token(id).await.unwrap();

    assert_eq!(actual, Some(expected));
}
#[sqlx::test(fixtures("linked_accounts", "tokens"))]
async fn read_nonexistent_token(pool: PgPool) {
    let id = "775316334259077120".parse().unwrap();

    let actual = pool.get_token(id).await.unwrap();

    assert_eq!(actual, None);
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn write_token(pool: PgPool) {
    let id = "755497867606622450".parse().unwrap();

    let token = Token {
        access_token: "access_token".into(),
        refresh_token: "refresh_token".into(),
        expires_at: OffsetDateTime::now_utc() + Duration::seconds(10),
    };

    let expected = token.clone();

    let actual = pool.write_token(id, token).await.unwrap();

    assert_eq!(actual, expected);
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn write_token_expired(pool: PgPool) {
    let id = "755497867606622450".parse().unwrap();

    let token = Token {
        access_token: "access_token".into(),
        refresh_token: "refresh_token".into(),
        expires_at: datetime!(2000-01-01 00:00:00 UTC),
    };

    let expected = token.clone();

    let actual = pool.write_token(id, token).await.unwrap();

    assert_eq!(actual, expected);
}

#[sqlx::test(fixtures("linked_accounts", "tokens"))]
async fn overwrite_existing_token(pool: PgPool) {
    let id = "755497867606622450".parse().unwrap();

    let token = Token {
        access_token: "access_token".into(),
        refresh_token: "refresh_token".into(),
        expires_at: datetime!(2000-01-01 00:00:00 UTC),
    };

    let expected = token.clone();

    let actual = pool.write_token(id, token).await.unwrap();

    assert_eq!(actual, expected);
}
