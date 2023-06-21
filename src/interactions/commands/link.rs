use twilight_mention::Mention;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::CommandOptionValue,
    },
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    database::Database,
    interactions::{
        components::code::{self, CustomId},
        context::ApplicationCommandInteraction,
        InteractionError,
    },
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

    let author_id = interaction.author_id().unwrap();
    let account_url = format!("[{}]({})", username, site::User::url(username.to_string()));

    let (db, scratch_api) = tokio::join!(
        state.pool.get_scratch_account(username.to_string()),
        state.client.get::<api::User>(username.to_string()),
    );

    if let Some(account) = db.unwrap() {
        let content = if account.id == author_id {
            locale.already_linked_to_you(&account_url)
        } else {
            locale.already_linked_to_other(&author_id.mention().to_string(), &account_url)
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
                locale.error_not_found_user(&account_url),
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

    let code_button = code::build(
        CustomId {
            username: username.to_string(),
            id: author_id,
        },
        locale,
    );

    return Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content(locale.link_your_account(&author_id.mention().to_string(), &account_url))
                .components([Component::ActionRow(ActionRow {
                    components: vec![
                        code_button,
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
                .allowed_mentions(Default::default())
                .build(),
        ),
    });
}
