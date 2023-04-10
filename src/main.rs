mod interactions;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use shuttle_secrets::SecretStore;

use interactions::interaction_handler;
use state::AppState;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new(secrets);

    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/interactions", post(interaction_handler))
        .with_state(state);

    Ok(router.into())
}
