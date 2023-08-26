use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use twilight_mention::Mention;
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
    database::Database,
    interactions::{components::done, context::MessageComponentInteraction, InteractionError},
    locales::Locale,
    scratch::site::user_link,
    state::AppState,
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
    state: AppState,
    interaction: MessageComponentInteraction,
    custom_id: CustomId,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    let author_id = interaction.author_id().unwrap();

    if author_id != custom_id.id {
        // TODO: respond with "not for you"
        todo!();
    }

    let already_linked = state
        .pool
        .get_scratch_account(custom_id.username.to_string())
        .await?;

    if let Some(account) = already_linked {
        let content = if account.id == author_id {
            locale.already_linked_to_you(&user_link(&custom_id.username))
        } else {
            locale.already_linked_to_other(
                &author_id.mention().to_string(),
                &user_link(&custom_id.username),
            )
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
