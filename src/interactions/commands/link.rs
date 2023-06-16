use twilight_mention::Mention;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::{CommandData, CommandOptionValue},
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    embed::EmbedBuilder,
    InteractionResponseDataBuilder,
};

use crate::{
    datastore::ScratchUser,
    embeds::Color,
    interactions::InteractionError,
    locales::{ExtendLocaleEmbed, Locale},
    scratch::{api, db, site, ScratchAPIError, Url},
    state::AppState,
};

pub fn register() -> Command {
    CommandBuilder::new("link", "Link your Scratch account", CommandType::ChatInput)
        .description_localizations(vec![("pl", "Połącz swoje konto Scratch")])
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

    let (db, scratch_api, scratch_db) = tokio::join!(
        sqlx::query!(
            r#"
                SELECT username, id
                FROM scratch_accounts
                WHERE lower(username) = lower($1)
            "#,
            username
        )
        .map(|row| ScratchUser {
            username: row.username,
            id: Id::<UserMarker>::new(row.id.parse().unwrap())
        })
        .fetch_optional(&state.pool),
        state.client.get::<api::User>(username.to_string()),
        state.client.get::<db::User>(username.to_string()),
    );

    if let Some(account) = db.unwrap() {
        account.id.mention();
    }

    let mut user_embed = EmbedBuilder::new().color(Color::Success.into());

    user_embed = match scratch_api {
        Ok(user) => {
            user_embed = user.extend_locale_embed(locale, user_embed);

            match scratch_db {
                Ok(user) => user.extend_locale_embed(locale, user_embed),
                Err(_) => user_embed,
            }
        }
        Err(error) => match error {
            ScratchAPIError::NotFound => user_embed
                .color(Color::Error.into())
                .title(locale.error_not_found())
                .description(locale.error_not_found_user(&site::User::url(username.to_string()))),
            ScratchAPIError::ServerError => user_embed
                .color(Color::Error.into())
                .title(locale.error_scratch_api())
                .description(locale.error_scratch_api_description()),
            ScratchAPIError::Other(_) => user_embed
                .color(Color::Error.into())
                .title(locale.error_internal())
                .description(locale.error_internal_description()),
        },
    };

    let embed = user_embed
        .validate()
        .expect("failed to validate embed")
        .build();

    let res = InteractionResponseDataBuilder::new()
        .embeds([embed])
        .build();

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(res),
    })
}
