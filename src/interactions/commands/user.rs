use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::{CommandData, CommandOptionValue},
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    embed::EmbedBuilder,
    InteractionResponseDataBuilder,
};

use crate::{
    embeds::Color,
    interactions::InteractionError,
    locales::{ExtendLocaleEmbed, Locale},
    scratch::{api, db, ScratchAPIError},
    state::AppState,
};

pub fn register() -> Command {
    CommandBuilder::new(
        "user",
        "Get info about a Scratch user",
        CommandType::ChatInput,
    )
    .description_localizations(vec![("pl", "Informacje o danym koncie Scratch")])
    .option(
        StringBuilder::new("username", "Account URL or username")
            .required(true)
            .description_localizations(vec![("pl", "Link do konta lub nazwa użytkownika")]),
    )
    .validate()
    .unwrap()
    .build()
}

pub async fn run(
    data: &Box<CommandData>,
    state: AppState,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    let username = match &data
        .options
        .iter()
        .find(|option| option.name == "username")
        .expect("option 'username' not found")
        .value
    {
        CommandOptionValue::String(value) => value,
        _ => unreachable!("expected option 'username' to be of type String"),
    };

    let (api_user, db_user) = tokio::join!(
        state.client.get::<api::User>(username.to_string()),
        state.client.get::<db::User>(username.to_string()),
    );

    let mut embed = EmbedBuilder::new().color(Color::Success.into());

    if let Ok(user) = api_user {
        embed = user.extend_locale_embed(locale, embed);
    }

    if let Ok(user) = db_user {
        embed = user.extend_locale_embed(locale, embed);
    }

    let embed = embed.validate().expect("failed to validate embed").build();

    let res = InteractionResponseDataBuilder::new()
        .embeds([embed])
        .build();

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(res),
    })
}
