use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use twilight_model::{
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, MessageFlags,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{
    interactions::{components::done, context::MessageComponentInteraction, InteractionError},
    locales::Locale,
};

use super::ComponentCustomId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomId {
    pub username: String,
    pub id: Id<UserMarker>,
}

pub fn build(custom_id: CustomId, locale: Locale) -> Component {
    Component::Button(Button {
        custom_id: ComponentCustomId::Code(custom_id).into(),
        disabled: false,
        emoji: None,
        label: Some(locale.generate_code()),
        style: ButtonStyle::Primary,
        url: None,
    })
}

pub async fn run(
    interaction: MessageComponentInteraction,
    custom_id: CustomId,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    if interaction.author_id().unwrap() != custom_id.id {
        return Err(InteractionError::NotImplemented);
    }

    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
    let generated = OffsetDateTime::now_utc();

    let done_button = done::build(
        done::CustomId {
            username: custom_id.username,
            code: code.to_owned(),
            generated,
        },
        locale,
    );

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content(code)
                .components([Component::ActionRow(ActionRow {
                    components: vec![done_button],
                })])
                .flags(MessageFlags::EPHEMERAL)
                .build(),
        ),
    })
}
