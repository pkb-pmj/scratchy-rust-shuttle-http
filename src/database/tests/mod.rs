use sqlx::PgPool;

use super::*;

/// Just to make sure it compiles with all the lifetimes
#[allow(dead_code)]
async fn lifetime_compile_test(pool: PgPool) {
    pool.get_scratch_account("username".into()).await.unwrap();

    let mut tx = pool.begin().await.unwrap();

    tx.get_scratch_account("username".into()).await.unwrap();

    pool.get_scratch_account("username".into()).await.unwrap();

    tx.get_scratch_account("username".into()).await.unwrap();

    tx.commit().await.unwrap();
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn get_account(pool: PgPool) {
    let account = pool
        .get_scratch_account("pmj_studio".to_string())
        .await
        .unwrap();

    assert_eq!(
        account,
        Some(ScratchAccount {
            id: "755497867606622450".parse().unwrap(),
            username: "PMJ_Studio".to_string()
        }),
        "case insensitive username",
    );

    let account = pool.get_scratch_account("a".to_string()).await.unwrap();

    assert_eq!(account, None, "nonexistent Scratch account");

    let account = pool
        .get_discord_account("755497867606622450".parse().unwrap())
        .await
        .unwrap();

    assert_eq!(
        account,
        Some(DiscordAccount {
            id: "755497867606622450".parse().unwrap()
        }),
        "Discord account",
    );

    let account = pool
        .get_discord_account("855497867606622450".parse().unwrap())
        .await
        .unwrap();

    assert_eq!(account, None, "nonexistent Discord account");

    let mut linked_accounts = pool
        .get_linked_scratch_accounts("755497867606622450".parse().unwrap())
        .await
        .unwrap();

    linked_accounts.sort_by_key(|account| account.username.to_string());

    assert_eq!(
        linked_accounts,
        vec![
            ScratchAccount {
                username: "PMJ_Studio".to_string(),
                id: "755497867606622450".parse().unwrap(),
            },
            ScratchAccount {
                username: "PMJ_test".to_string(),
                id: "755497867606622450".parse().unwrap(),
            }
        ],
        "linked Scratch accounts",
    );
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn create_discord_account(pool: PgPool) {
    pool.create_discord_account("755497867606622450".parse().unwrap())
        .await
        .expect_err("can't create account with already used ID");

    let created_account = pool
        .create_discord_account("855497867606622450".parse().unwrap())
        .await
        .unwrap();

    assert_eq!(
        created_account,
        DiscordAccount {
            id: "855497867606622450".parse().unwrap()
        },
        "create Discord account",
    );

    let read_account = pool
        .get_discord_account("855497867606622450".parse().unwrap())
        .await
        .unwrap();

    assert_eq!(
        read_account,
        Some(created_account),
        "successfully created Discord account",
    );
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn create_linked_scratch_account(pool: PgPool) {
    pool.create_linked_scratch_account(
        "PMJ_Studio".to_string(),
        "755497867606622450".parse().unwrap(),
    )
    .await
    .expect_err("can't link already linked account to the same user");

    pool.create_linked_scratch_account(
        "PMJ_MJBCS27".to_string(),
        "755497867606622450".parse().unwrap(),
    )
    .await
    .expect_err("can't link already linked account to other user");

    pool.create_linked_scratch_account(
        "PMJ_JPB14".to_string(),
        "855497867606622450".parse().unwrap(),
    )
    .await
    .expect_err("can't link to a nonexistent user");

    let linked_account = pool
        .create_linked_scratch_account(
            "PMJ_JPB14".to_string(),
            "755497867606622450".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        linked_account,
        ScratchAccount {
            username: "PMJ_JPB14".to_string(),
            id: "755497867606622450".parse().unwrap(),
        },
        "successfully linked Scratch account",
    );

    let mut linked_accounts = pool
        .get_linked_scratch_accounts("755497867606622450".parse().unwrap())
        .await
        .unwrap();

    linked_accounts.sort_by_key(|account| account.username.to_string());

    assert_eq!(
        linked_accounts,
        vec![
            ScratchAccount {
                username: "PMJ_JPB14".to_string(),
                id: "755497867606622450".parse().unwrap(),
            },
            ScratchAccount {
                username: "PMJ_Studio".to_string(),
                id: "755497867606622450".parse().unwrap(),
            },
            ScratchAccount {
                username: "PMJ_test".to_string(),
                id: "755497867606622450".parse().unwrap(),
            },
        ],
        "linked Scratch accounts",
    );
}
