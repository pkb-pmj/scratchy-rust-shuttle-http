use twilight_mention::Mention;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::{CommandData, CommandOptionValue},
    },
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::UserMarker, Id},
    user::User,
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    datastore::ScratchUser,
    interactions::InteractionError,
    locales::Locale,
    scratch::{api, site, ScratchAPIError, Url, STUDIO_URL},
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
    author: User,
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

    let (db, scratch_api) = tokio::join!(
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
    );

    if let Some(account) = db.unwrap() {
        let account_url = site::User::url(username.to_string());

        let content = if account.id == author.id {
            locale.already_linked_to_you(&account_url)
        } else {
            locale.already_linked_to_other(&account.id.mention().to_string(), &account_url)
        };

        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(content)
                    .build(),
            ),
        });
    }

    if let Err(error) = scratch_api {
        let (title, description) = match error {
            ScratchAPIError::NotFound => (
                locale.error_not_found(),
                locale.error_not_found_user(&site::User::url(username.to_string())),
            ),
            ScratchAPIError::ServerError => (
                locale.error_scratch_api(),
                locale.error_scratch_api_description(),
            ),
            ScratchAPIError::Other(_) => {
                (locale.error_internal(), locale.error_internal_description())
            }
        };

        let content = format!("## {title}\n{description}");

        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(content)
                    .build(),
            ),
        });
    }

    return Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content(locale.already_linked_to_you(username))
                .components([Component::ActionRow(ActionRow {
                    components: vec![
                        Component::Button(Button {
                            custom_id: Some(format!("code {username}").into()),
                            disabled: false,
                            emoji: None,
                            label: Some(locale.generate_code()),
                            style: ButtonStyle::Primary,
                            url: None,
                        }),
                        Component::Button(Button {
                            custom_id: None,
                            disabled: false,
                            emoji: None,
                            label: Some(locale.go_to_studio()),
                            style: ButtonStyle::Link,
                            url: Some(STUDIO_URL.into()),
                        }),
                    ],
                })])
                .build(),
        ),
    });
}
