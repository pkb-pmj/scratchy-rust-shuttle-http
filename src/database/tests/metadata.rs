use time::macros::datetime;

use super::*;

#[sqlx::test(fixtures("linked_accounts", "metadata"))]
async fn get_oldest_metadata(pool: PgPool) {
    let expected = Some((
        "755497867606622450".parse().unwrap(),
        datetime!(2023-08-03 12:00:00 UTC),
    ));

    let actual = pool.get_oldest_metadata().await.unwrap();

    assert_eq!(actual, expected);
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn get_oldest_metadata_none(pool: PgPool) {
    let actual = pool.get_oldest_metadata().await.unwrap();

    assert_eq!(actual, None);
}

#[sqlx::test(fixtures("linked_accounts", "metadata"))]
async fn get_metadata(pool: PgPool) {
    let expected = Some(RoleConnectionData {
        scratcher: true,
        followers: 1000,
        joined: datetime!(2020-08-03 12:00:00 UTC),
    });

    let actual = pool
        .get_metadata("755497867606622450".parse().unwrap())
        .await
        .unwrap();

    assert_eq!(actual, expected);
}

#[sqlx::test(fixtures("linked_accounts", "metadata"))]
async fn write_metadata(pool: PgPool) {
    let expected = RoleConnectionData {
        scratcher: true,
        followers: 1001,
        joined: datetime!(2020-08-03 12:00:00 UTC),
    };

    let actual = pool
        .write_metadata("755497867606622450".parse().unwrap(), &expected)
        .await
        .unwrap();

    assert_eq!(actual, expected);
}
