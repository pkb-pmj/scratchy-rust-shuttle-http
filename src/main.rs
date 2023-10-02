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
use tracing::{debug, error, info, trace, warn};
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
    tracing_subscriber::fmt()
        .with_env_filter("info,scratchy=trace")
        .init();
    debug!("tracing initialized");

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");

    debug!("setting panic hook");
    std::panic::set_hook(Box::new(panic_hook));

    debug!("initializing app state");
    let state = AppState::new(secrets, pool);

    debug!("running migrations");
    sqlx::migrate!("./migrations")
        .run(&state.pool)
        .await
        .expect("database migration failed");

    debug!("registering commands");
    register_commands(state.config.token.to_string())
        .await
        .expect("failed to register commands");

    debug!("registering metadata");
    register_metadata(&state)
        .await
        .expect("failed to register metadata");

    debug!("creating router");
    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/interactions", post(interaction_handler))
        .merge(linked_roles::router())
        .with_state(state.clone());

    debug!("spawning background metadata updater");
    spawn_background_updater(state);

    debug!("returning router");
    Ok(router.into())
}
