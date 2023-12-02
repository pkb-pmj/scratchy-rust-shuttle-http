use tracing::debug;
use twilight_model::{
    application::command::{Command, CommandType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    embeds::{Color, Extend, User},
    interactions::{
        context::{ApplicationCommandInteraction, GetOption},
        InteractionError,
    },
    locales::{Locale, ToLocalized},
    scratch::{
        api::ScratchAPIClient,
        db::ScratchDBClient,
        site::{extract_username, user_link},
    },
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
    let username: &String = interaction.data().options.get_option("username")?;

    let Some(username) = extract_username(username) else {
        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(locale.invalid_username())
                    .build(),
            ),
        });
    };

    let (api, db) = tokio::join!(
        state.reqwest_client.get_scratch_api_user(&username),
        state.reqwest_client.get_scratch_db_user(&username),
    );

    let response = match api? {
        Some(api) => {
            let mut user = User::new();
            debug!(?api);
            user.extend(api);

            if let Ok(Some(db)) = db {
                debug!(?db);
                user.extend(db);
            }
            debug!(?user);

            let embed = user
                .to_localized(locale)
                .color(Color::Success.into())
                .validate()?
                .build();

            InteractionResponseDataBuilder::new().embeds([embed])
        }
        None => InteractionResponseDataBuilder::new()
            .content(locale.user_not_found(&user_link(&username))),
    };

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(response.build()),
    })
}
