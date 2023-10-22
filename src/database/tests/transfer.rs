use super::*;

#[sqlx::test(fixtures("linked_accounts"))]
async fn transfer_linked_accounts_already_linked(pool: PgPool) {
    let result = transfer_linked_accounts(
        &pool,
        "PMJ_Studio".to_string(),
        "755497867606622450".parse().unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(result, Err(TransferError::AlreadyLinkedToYou));
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn transfer_linked_accounts_not_linked(pool: PgPool) {
    let result = transfer_linked_accounts(
        &pool,
        "PMJ_JPB14".to_string(),
        "755497867606622450".parse().unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(result, Err(TransferError::NotLinked));
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn transfer_linked_accounts_create_discord_account(pool: PgPool) {
    let (id, mut accounts) = transfer_linked_accounts(
        &pool,
        "PMJ_Studio".to_string(),
        "775316334259077121".parse().unwrap(),
    )
    .await
    .unwrap()
    .unwrap();

    accounts.sort();

    assert_eq!(id, 755497867606622450u64);
    assert_eq!(accounts, vec!["PMJ_Studio", "PMJ_test"]);
}

#[sqlx::test(fixtures("linked_accounts"))]
async fn transfer_linked_accounts_ok(pool: PgPool) {
    let (id, mut accounts) = transfer_linked_accounts(
        &pool,
        "PMJ_Studio".to_string(),
        "775316334259077120".parse().unwrap(),
    )
    .await
    .unwrap()
    .unwrap();

    accounts.sort();

    assert_eq!(id, 755497867606622450u64);
    assert_eq!(accounts, vec!["PMJ_Studio", "PMJ_test"]);
}
