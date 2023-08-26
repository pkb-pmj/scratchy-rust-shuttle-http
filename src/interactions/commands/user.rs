use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::CommandOptionValue,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    embeds::{Color, Extend, User},
    interactions::{context::ApplicationCommandInteraction, InteractionError},
    locales::{Locale, ToLocalized},
    scratch::{api::ScratchAPIClient, db::ScratchDBClient, site::user_link},
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
    state: AppState,
    interaction: ApplicationCommandInteraction,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    let username = match &interaction
        .data()
        .options
        .iter()
        .find(|option| option.name == "username")
        .expect("option 'username' not found")
        .value
    {
        CommandOptionValue::String(value) => value,
        _ => unreachable!("expected option 'username' to be of type String"),
    };

    let (api, db) = tokio::join!(
        state.reqwest_client.get_scratch_api_user(username),
        state.reqwest_client.get_scratch_db_user(username),
    );

    let response = match api? {
        Some(api) => {
            let mut user = User::new();
            user.extend(api);

            if let Ok(Some(db)) = db {
                user.extend(db);
            }

            let embed = user
                .to_localized(locale)
                .color(Color::Success.into())
                .validate()
                .expect("failed to validate embed")
                .build();

            InteractionResponseDataBuilder::new().embeds([embed])
        }
        None => InteractionResponseDataBuilder::new()
            .content(locale.user_not_found(&user_link(username))),
    };

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(response.build()),
    })
}
