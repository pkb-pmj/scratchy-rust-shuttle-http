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
    scratch::{
        api::ScratchAPIClient,
        site::{user_link, username_is_valid},
        STUDIO_URL,
    },
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
    let mut username = match &interaction
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

    if !username_is_valid(username) {
        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(locale.invalid_username())
                    .build(),
            ),
        });
    }

    let author_id = interaction.author_id().unwrap();

    let (db, scratch_api) = tokio::join!(
        state.pool.get_scratch_account(username.to_string()),
        state.reqwest_client.get_scratch_api_user(&username),
    );

    if let Some(account) = db.unwrap() {
        username = &account.username;

        let content = if account.id == author_id {
            locale.already_linked_to_you(&user_link(username))
        } else {
            locale.already_linked_to_other(&account.id.mention().to_string(), &user_link(username))
        };

        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(content)
                    .allowed_mentions(Default::default())
                    .build(),
            ),
        });
    }

    let scratch_api = scratch_api?;
    match scratch_api {
        Some(ref user) => username = &user.username,
        None => {
            let content = format!(
                "## {}\n{}",
                locale.error_not_found(),
                locale.error_not_found_user(&user_link(username))
            );

            return Ok(InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .content(content)
                        .build(),
                ),
            });
        }
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
                .content(
                    locale
                        .link_your_account(&author_id.mention().to_string(), &user_link(username)),
                )
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
