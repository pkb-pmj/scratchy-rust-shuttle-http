use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::{CommandData, CommandOptionValue},
    },
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::{
    interactions::InteractionError,
    scratch::{api, get, ScratchAPIError},
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

    let user: api::User = match get(state.client, api::User::url(username)).await {
        Ok(value) => value,
        Err(err) => {
            return Ok(InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionResponseData {
                    content: Some(match err {
                        ScratchAPIError::NotFound => "Not found".into(),
                        ScratchAPIError::Other(_) => "Scratch API error".into(),
                    }),
                    ..Default::default()
                }),
            })
        }
    };

    let embed = EmbedBuilder::new()
        .author(
            EmbedAuthorBuilder::new(&user.username)
                .url(format!("https://scratch.mit.edu/users/{}", &user.username))
                .icon_url(ImageSource::url(user.profile.images.n50x50).unwrap()),
        )
        .color(0xcc6600)
        .field(EmbedFieldBuilder::new("About me", user.profile.bio))
        .field(EmbedFieldBuilder::new(
            "What I'm working on",
            user.profile.status,
        ))
        .validate()
        .unwrap()
        .build();

    let res = InteractionResponseDataBuilder::new()
        .embeds([embed])
        .build();

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(res),
    })
}
