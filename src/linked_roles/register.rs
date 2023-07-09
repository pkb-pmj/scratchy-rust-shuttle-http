use crate::state::AppState;

use super::{client::RoleConnectionClient, metadata::metadata, model::Metadata};

fn key(metadata: &Metadata) -> String {
    metadata.key.to_owned()
}

pub async fn register_metadata(state: &AppState) -> Result<(), reqwest::Error> {
    let mut old_metadata = state
        .reqwest_client
        .get_metadata(&state.config.client_id, &state.config.token)
        .await?;
    old_metadata.sort_by_key(key);

    let mut new_metadata = metadata();
    new_metadata.sort_by_key(key);

    if old_metadata == new_metadata {
        return Ok(());
    }

    state
        .reqwest_client
        .put_metadata(&state.config.client_id, &state.config.token, new_metadata)
        .await?;

    Ok(())
}
