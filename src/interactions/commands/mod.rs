use twilight_model::http::interaction::InteractionResponse;

use crate::{locales::Locale, state::AppState};

use super::{context::ApplicationCommandInteraction, InteractionError};

pub mod about;
pub mod link;
pub mod ping;
pub mod user;

pub async fn router(
    state: AppState,
    interaction: ApplicationCommandInteraction,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    match interaction.data().name.as_str() {
        "about" => about::run().await,
        "link" => link::run(state, interaction, locale).await,
        "ping" => ping::run().await,
        "user" => user::run(state, interaction, locale).await,
        _ => unimplemented!(),
    }
}
