use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

use crate::interactions::InteractionError;

pub async fn run() -> Result<InteractionResponse, InteractionError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some("Pong!".into()),
            ..Default::default()
        }),
    })
}
