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
