mod interactions;

use axum::{
    routing::{get, post},
    Router,
};
use ed25519_dalek::PublicKey;
use interactions::handle_interaction;
use shuttle_secrets::SecretStore;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let discord_public_key =
        PublicKey::from_bytes(&hex::decode(secrets.get("discord_public_key").unwrap()).unwrap())
            .unwrap();

    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/interactions", post(handle_interaction))
        .with_state(discord_public_key);

    Ok(router.into())
}
