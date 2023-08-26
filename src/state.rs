use axum::extract::FromRef;
use ed25519_dalek::PublicKey;
use oauth2::basic::BasicClient;
use reqwest::{Client, Url};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use time::OffsetDateTime;

use crate::{embeds::timestamp, linked_roles::create_oauth_client};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub oauth_client: BasicClient,
    pub reqwest_client: Client,
    pub pool: PgPool,
    pub start_time: StartTime,
}

impl AppState {
    pub fn new(secrets: SecretStore, pool: PgPool) -> Self {
        let config = Config::new(secrets);

        let oauth_client = create_oauth_client(&config);

        let reqwest_client = Client::new();

        let start_time = StartTime::new();

        Self {
            config,
            oauth_client,
            reqwest_client,
            pool,
            start_time,
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

impl FromRef<AppState> for BasicClient {
    fn from_ref(input: &AppState) -> Self {
        input.oauth_client.clone()
    }
}

impl FromRef<AppState> for Client {
    fn from_ref(input: &AppState) -> Self {
        input.reqwest_client.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

#[derive(Debug, Clone)]
pub struct StartTime(OffsetDateTime);

impl StartTime {
    fn new() -> Self {
        Self(OffsetDateTime::now_utc())
    }

    pub fn timestamp(&self) -> String {
        timestamp(self.0)
    }
}

impl FromRef<AppState> for StartTime {
    fn from_ref(input: &AppState) -> Self {
        input.start_time.clone()
    }
}
