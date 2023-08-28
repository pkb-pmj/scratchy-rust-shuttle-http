use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::CommandOptionValue,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    InteractionResponseDataBuilder,
};

use crate::{
    embeds::{Color, Extend, Project},
    interactions::{context::ApplicationCommandInteraction, InteractionError},
    locales::{Locale, ToLocalized},
    scratch::{
        api::ScratchAPIClient,
        db::ScratchDBClient,
        site::{extract_project_id, project_link},
    },
    state::AppState,
};

pub fn register() -> Command {
    CommandBuilder::new(
        "project",
        "Get info about a Scratch project",
        CommandType::ChatInput,
    )
    .description_localizations(vec![("pl", "Informacje o danym projekcie Scratch")])
    .option(
        StringBuilder::new("id", "Project URL or ID")
            .required(true)
            .description_localizations(vec![("pl", "Link lub ID projektu")]),
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
    let id = match &interaction
        .data()
        .options
        .iter()
        .find(|option| option.name == "id")
        .expect("option 'id' not found")
        .value
    {
        CommandOptionValue::String(value) => value,
        _ => unreachable!("expected option 'id' to be of type String"),
    };

    let Some(id) = extract_project_id(id) else {
        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(locale.invalid_project_id())
                    .build(),
            ),
        });
    };

    let (api, db) = tokio::join!(
        state.reqwest_client.get_scratch_api_project(id),
        state.reqwest_client.get_scratch_db_project(id),
    );

    let response = match api? {
        Some(api) => {
            let mut project = Project::new();
            project.extend(api);

            if let Ok(Some(db)) = db {
                project.extend(db);
            }

            let embed = project
                .to_localized(locale)
                .color(Color::Success.into())
                .validate()?
                .build();

            InteractionResponseDataBuilder::new().embeds([embed])
        }
        None => InteractionResponseDataBuilder::new()
            .content(locale.project_not_found(&project_link(id))),
    };

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(response.build()),
    })
}
