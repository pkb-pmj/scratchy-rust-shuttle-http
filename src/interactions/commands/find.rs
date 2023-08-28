use std::fmt::Write;

use twilight_mention::Mention;
use twilight_model::{
    application::command::{Command, CommandType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder, SubCommandBuilder, UserBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    database::Database,
    interactions::{
        context::{ApplicationCommandInteraction, GetOption, GetSubcommand},
        InteractionError,
    },
    locales::Locale,
    scratch::site::{extract_username, user_link},
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
    let (subcommand, options) = interaction.data().options.get_subcommand()?;

    let id = match subcommand {
        "by-scratch" => {
            let username: &String = options.get_option("username")?;

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

            if let Some(scratch_account) =
                state.pool.get_scratch_account(username.to_string()).await?
            {
                scratch_account.id
            } else {
                return Ok(InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content(locale.no_linked_discord_account(&user_link(&username)))
                            .build(),
                    ),
                });
            }
        }
        "by-discord" => *options.get_option("user")?,
        _ => panic!("unknown subcommand name"),
    };

    let mention = id.mention().to_string();

    let linked_accounts = state.pool.get_linked_scratch_accounts(id).await?;

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
