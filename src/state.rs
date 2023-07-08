use axum::extract::FromRef;
use ed25519_dalek::PublicKey;
use reqwest::Url;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

use crate::scratch::ScratchClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub scratch_client: ScratchClient,
    pub pool: PgPool,
}

impl AppState {
    pub fn new(secrets: SecretStore, pool: PgPool) -> Self {
        let config = Config::new(secrets);

        let scratch_client = ScratchClient::new();

        Self {
            config,
            scratch_client,
            pool,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub redirect_url: Url,
    pub client_id: String,
    pub client_secret: String,
    pub public_key: PublicKey,
    pub token: String,
}

impl Config {
    pub fn new(secrets: SecretStore) -> Self {
        let mut redirect_url: Url = secrets
            .get("base_url")
            .expect("missing base_url")
            .parse()
            .expect("invalid base_url");
        redirect_url.set_path("discord-oauth-callback");

        let client_id = secrets
            .get("discord_client_id")
            .expect("missing discord_client_id");

        let client_secret = secrets
            .get("discord_client_secret")
            .expect("missing discord_client_secret");

        let public_key = secrets
            .get("discord_public_key")
            .expect("missing discord_public_key");
        let public_key =
            &hex::decode(public_key).expect("discord_public_key is not a valid hex string");
        let public_key = PublicKey::from_bytes(&public_key)
            .expect("discord_public_key is not a valid public key");

        let token = secrets.get("discord_token").expect("missing discord_token");

        Self {
            redirect_url,
            client_id,
            client_secret,
            public_key,
            token,
        }
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

impl FromRef<AppState> for ScratchClient {
    fn from_ref(input: &AppState) -> Self {
        input.scratch_client.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}
