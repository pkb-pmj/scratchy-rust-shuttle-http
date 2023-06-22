use std::fmt::Write;

use twilight_mention::Mention;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::CommandOptionValue,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder, SubCommandBuilder, UserBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    database::Database,
    interactions::{context::ApplicationCommandInteraction, InteractionError},
    locales::Locale,
    scratch::site::{user_link, username_is_valid},
    state::AppState,
};

pub fn register() -> Command {
    CommandBuilder::new("find", "Find linked accounts", CommandType::ChatInput)
        .description_localizations(vec![("pl", "Znajdź połączone konta")])
        .option(
            SubCommandBuilder::new("by-scratch", "Find linked accounts of a Scratch user").option(
                StringBuilder::new("username", "Scratch account URL or username")
                    .required(true)
                    .description_localizations(vec![(
                        "pl",
                        "Link do konta Scratch lub nazwa użytkownika",
                    )]),
            ),
        )
        .option(
            SubCommandBuilder::new("by-discord", "Find linked accounts of a Discord user").option(
                UserBuilder::new("user", "Discord account")
                    .required(true)
                    .description_localizations(vec![("pl", "Konto Discord")]),
            ),
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
    let data = interaction.data();
    if data.options.len() != 1 {
        panic!("expected exactly one option - subcommand");
    }
    let subcommand = &data.options[0];

    let options = match &subcommand.value {
        CommandOptionValue::SubCommand(options) => options,
        _ => panic!("expected option to be of type SubCommand"),
    };

    let id = match subcommand.name.as_str() {
        "by-scratch" => {
            assert!(options.len() == 1, "expected exactly one option");
            assert_eq!(options[0].name, "username", "expected option 'username'");

            let username = match &options[0].value {
                CommandOptionValue::String(username) => username,
                _ => panic!("expected option 'username' to be of type String"),
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

            if let Some(scratch_account) = state
                .pool
                .get_scratch_account(username.to_string())
                .await
                .unwrap()
            {
                scratch_account.id
            } else {
                return Ok(InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content(locale.no_linked_discord_account(&user_link(username)))
                            .build(),
                    ),
                });
            }
        }
        "by-discord" => {
            assert!(options.len() == 1, "expected exactly one option");
            assert_eq!(options[0].name, "user", "expected option 'user'");

            match &options[0].value {
                CommandOptionValue::User(id) => *id,
                _ => panic!("expected option 'user' to be of type User"),
            }
        }
        _ => panic!("unknown subcommand name"),
    };

    let mention = id.mention().to_string();

    let linked_accounts = state.pool.get_linked_scratch_accounts(id).await.unwrap();

    if linked_accounts.len() == 0 {
        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(locale.no_linked_scratch_accounts(&mention))
                    .allowed_mentions(Default::default())
                    .build(),
            ),
        });
    }

    let mut content = locale.linked_accounts(&mention);

    for account in linked_accounts {
        content.write_str("\n- ").unwrap();
        content.write_str(&user_link(&account.username)).unwrap();
    }

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
