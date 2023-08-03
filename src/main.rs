mod database;
mod embeds;
mod interactions;
mod linked_roles;
mod locales;
mod scratch;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use linked_roles::spawn_background_updater;
use shuttle_secrets::SecretStore;

use interactions::{interaction_handler, register::register_commands};
use sqlx::PgPool;
use state::AppState;
use tracing_panic::panic_hook;

use crate::linked_roles::register_metadata;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_aws_rds::Postgres(local_uri = "{secrets.database_url}")] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    std::panic::set_hook(Box::new(panic_hook));

    let state = AppState::new(secrets, pool);

    sqlx::migrate!("./migrations")
        .run(&state.pool)
        .await
        .expect("database migration failed");

    register_commands(state.config.token.to_string())
        .await
        .expect("failed to register commands");

    register_metadata(&state)
        .await
        .expect("failed to register metadata");

    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/interactions", post(interaction_handler))
        .merge(linked_roles::router())
        .with_state(state.clone());

    spawn_background_updater(state);

    Ok(router.into())
}
