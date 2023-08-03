mod discord_scratch;
mod token;

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
