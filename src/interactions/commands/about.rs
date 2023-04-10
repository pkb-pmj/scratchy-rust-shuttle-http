use twilight_model::{
    application::command::{Command, CommandType},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

use crate::interactions::InteractionError;

pub fn register() -> Command {
    CommandBuilder::new(
        "about",
        "General info about the bot",
        CommandType::ChatInput,
    )
    .description_localizations(vec![("pl", "Ogólne informacje o bocie")])
    .validate()
    .unwrap()
    .build()
}

pub async fn run() -> Result<InteractionResponse, InteractionError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some("About Scratchy".into()),
            ..Default::default()
        }),
    })
}
