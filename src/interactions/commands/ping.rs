use twilight_model::{
    application::command::{Command, CommandType},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

use crate::interactions::InteractionError;

pub fn register() -> Command {
    CommandBuilder::new("ping", "Current bot status", CommandType::ChatInput)
        .description_localizations(vec![("pl", "Aktualny stan bota")])
        .validate()
        .unwrap()
        .build()
}

pub async fn run() -> Result<InteractionResponse, InteractionError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some("Pong!".into()),
            ..Default::default()
        }),
    })
}
