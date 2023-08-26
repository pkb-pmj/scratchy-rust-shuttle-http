use tracing::{debug_span, Instrument};
use twilight_model::http::interaction::InteractionResponse;

use crate::{locales::Locale, state::AppState};

use super::{context::ApplicationCommandInteraction, InteractionError};

pub mod about;
pub mod find;
pub mod link;
pub mod ping;
pub mod project;
pub mod user;

pub async fn router(
    state: AppState,
    interaction: ApplicationCommandInteraction,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    let span = debug_span!(
        "command",
        command = interaction.data().name,
        user = %interaction.author_id().unwrap(),
        guild = ?interaction.guild_id.map(|v| v.get()),
        channel = ?interaction.channel_id.map(|v| v.get()),
    );

    async move {
        match interaction.data().name.as_str() {
            "about" => about::run().await,
            "find" => find::run(state, interaction, locale).await,
            "link" => link::run(state, interaction, locale).await,
            "ping" => ping::run(state, locale).await,
            "project" => project::run(state, interaction, locale).await,
            "user" => user::run(state, interaction, locale).await,
            command => Err(InteractionError::UnknownCommand(command.to_string())),
        }
    }
    .instrument(span)
    .await
}
