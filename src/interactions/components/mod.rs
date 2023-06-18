pub mod code;
pub mod done;

use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    http::interaction::InteractionResponse,
};

use crate::{locales::Locale, state::AppState};

use super::{context::InteractionContext, InteractionError};

pub async fn router(
    state: AppState,
    interaction: InteractionContext<MessageComponentInteractionData>,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    let (custom_id, _) = interaction
        .data()
        .custom_id
        .as_str()
        .split_once(' ')
        .expect("Invalid custom_id");

    match custom_id {
        "code" => code::run(interaction, locale).await,
        _ => unimplemented!(),
    }
}
