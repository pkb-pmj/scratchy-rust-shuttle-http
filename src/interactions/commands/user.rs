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

    let (api, db) = tokio::join!(
        state.client.get::<api::User>(username.to_string()),
        state.client.get::<db::User>(username.to_string()),
    );

    let mut embed = EmbedBuilder::new().color(Color::Success.into());

    // embed = match result {
    //     (Err(ScratchAPIError::NotFound), Err(ScratchAPIError::NotFound)) => embed
    //         .title(locale.error_not_found())
    //         .description(
    //             locale.error_not_found_user(&format!("https://scratch.mit.edu/user/{username}")),
    //         )
    //         .footer(APIStatus::Both),
    //     (Err(ScratchAPIError::Other(_)), Err(ScratchAPIError::Other(_))) => {
    //         embed.footer(APIStatus::None)
    //     }
    //     (Ok(api), Err(ScratchAPIError::Other(_))) => api
    //         .extend_locale_embed(locale, embed)
    //         .footer(APIStatus::OnlyAPI),
    //     (Err(ScratchAPIError::Other(_)), Ok(db)) => db
    //         .extend_locale_embed(locale, embed)
    //         .footer(APIStatus::OnlyDB),
    //     (api, db) => {
    //         if let Ok(user) = api {
    //             embed = user.extend_locale_embed(locale, embed);
    //         }
    //         if let Ok(user) = db {
    //             embed = user.extend_locale_embed(locale, embed);
    //         }
    //         embed
    //     } // (Ok(api), Ok(db)) => db
    //       //     .extend_locale_embed(locale, api.extend_locale_embed(locale, embed))
    //       //     .footer(APIStatus::Both),
    //       // (Ok(api), Err(db)) => api
    //       //     .extend_locale_embed(locale, embed)
    //       //     .footer(APIStatus::OnlyAPI),
    //       // (Err(api), Ok(db)) => db
    //       //     .extend_locale_embed(locale, embed)
    //       //     .footer(APIStatus::OnlyDB),
    //       // (Err(api), Err(db)) => {
    //       //     embed = embed.color(Color::Error.into());
    //       //     match (api, db) {
    //       //         (ScratchAPIError::Other(_), ScratchAPIError::Other(_)) => {
    //       //             embed.footer(APIStatus::None)
    //       //         }
    //       //         _ => embed
    //       //             .title(locale.error_not_found())
    //       //             .description(locale.error_not_found_user(&format!(
    //       //                 "https://scratch.mit.edu/user/{username}"
    //       //             )))
    //       //             .footer(APIStatus::Both),
    //       //     }
    //       // }
    // };
    embed = match api {
        Ok(user) => {
            embed = user.extend_locale_embed(locale, embed);

            match db {
                Ok(user) => user.extend_locale_embed(locale, embed),
                Err(_) => embed,
            }
        }
        Err(error) => match error {
            ScratchAPIError::NotFound => embed
                .color(Color::Error.into())
                .title(locale.error_not_found())
                .description(
                    locale
                        .error_not_found_user(&format!("https://scratch.mit.edu/user/{username}")),
                ),
            ScratchAPIError::ServerError => embed
                .color(Color::Error.into())
                .title(locale.error_scratch_api())
                .description(locale.error_scratch_api_description()),
            ScratchAPIError::Other(_) => embed
                .color(Color::Error.into())
                .title(locale.error_internal())
                .description(locale.error_internal_description()),
        },
    };

    let embed = embed.validate().expect("failed to validate embed").build();

    let res = InteractionResponseDataBuilder::new()
        .embeds([embed])
        .build();

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(res),
    })
}
