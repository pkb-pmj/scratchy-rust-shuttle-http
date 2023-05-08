use ed25519_dalek::PublicKey;
use shuttle_secrets::SecretStore;

use crate::scratch::ScratchClient;

#[derive(Clone)]
pub struct AppState {
    pub discord_public_key: PublicKey,
    pub discord_token: String,
    pub client: ScratchClient,
}

impl AppState {
    pub fn new(secrets: SecretStore) -> Self {
        let discord_public_key = PublicKey::from_bytes(
            &hex::decode(secrets.get("discord_public_key").unwrap()).unwrap(),
        )
        .unwrap();

        let discord_token = secrets.get("discord_token").unwrap();

        let client = ScratchClient::new();

        Self {
            discord_public_key,
            discord_token,
            client,
        }
    }
}
