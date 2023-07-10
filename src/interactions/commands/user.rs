use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::CommandOptionValue,
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
    interactions::{context::ApplicationCommandInteraction, InteractionError},
    locales::{ExtendLocaleEmbed, Locale},
    scratch::{api::ScratchAPIClient, db::ScratchDBClient, site::user_link, ScratchAPIError},
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

    let response = match api {
        Ok(user) => {
            let mut embed = EmbedBuilder::new().color(Color::Success.into());

            embed = user.extend_locale_embed(locale, embed);

            if let Ok(user) = db {
                embed = user.extend_locale_embed(locale, embed)
            }

            let embed = embed.validate().expect("failed to validate embed").build();

            InteractionResponseDataBuilder::new().embeds([embed])
        }
        Err(error) => {
            let (title, description) = match error {
                ScratchAPIError::NotFound => (
                    locale.error_not_found(),
                    locale.error_not_found_user(&user_link(username)),
                ),
                ScratchAPIError::ServerError => (
                    locale.error_scratch_api(),
                    locale.error_scratch_api_description(),
                ),
                ScratchAPIError::Other(_) => {
                    (locale.error_internal(), locale.error_internal_description())
                }
            };

            InteractionResponseDataBuilder::new().content(format!("{}\n{}", title, description))
        }
    };

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(response.build()),
    })
}
