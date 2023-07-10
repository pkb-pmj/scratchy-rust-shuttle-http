use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use crate::{database::Database, linked_roles::Token};

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
        expires_at: OffsetDateTime::now_utc() - Duration::seconds(10),
    };

    let expected = token.clone();

    let actual = pool.write_token(id, token).await.unwrap();

    assert_eq!(actual.access_token, expected.access_token);
    assert_eq!(actual.refresh_token, expected.refresh_token);
    assert!(actual.expires_at < OffsetDateTime::now_utc());
}
