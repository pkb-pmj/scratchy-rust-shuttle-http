use rand::distributions::{Alphanumeric, DistString};
use time::OffsetDateTime;
use twilight_model::{
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{
    interactions::{components::done, context::MessageComponentInteraction, InteractionError},
    locales::Locale,
};

pub fn build(username: String, locale: Locale) -> Component {
    Component::Button(Button {
        custom_id: Some(format!("code {username}").into()),
        disabled: false,
        emoji: None,
        label: Some(locale.generate_code()),
        style: ButtonStyle::Primary,
        url: None,
    })
}

pub async fn run(
    interaction: MessageComponentInteraction,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    let username = interaction.data().custom_id.strip_prefix("code ").unwrap();

    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);

    let timestamp = OffsetDateTime::now_utc();

    let done_button = done::build(username.to_string(), code.to_owned(), timestamp, locale);

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content(code)
                .components([Component::ActionRow(ActionRow {
                    components: vec![done_button],
                })])
                .build(),
        ),
    })
}
