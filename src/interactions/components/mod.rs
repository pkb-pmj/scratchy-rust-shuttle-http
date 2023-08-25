pub mod code;
pub mod done;

use std::{fmt::Display, str::FromStr};

use base64::{display::Base64Display, engine::general_purpose::STANDARD, Engine};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug_span, Instrument};
use twilight_model::http::interaction::InteractionResponse;

use crate::{locales::Locale, state::AppState};

use super::{context::MessageComponentInteraction, InteractionError};

pub async fn router(
    state: AppState,
    interaction: MessageComponentInteraction,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    let custom_id: ComponentCustomId = interaction.data().custom_id.parse()?;

    let span = debug_span!(
        "component",
        ?custom_id,
        user = %interaction.author_id().unwrap(),
        guild = ?interaction.guild_id.map(|v| v.get()),
        channel = ?interaction.channel_id.map(|v| v.get()),
    );

    async move {
        match custom_id {
            ComponentCustomId::Code(custom_id) => {
                code::run(state, interaction, custom_id, locale).await
            }
            ComponentCustomId::Done(custom_id) => {
                done::run(state, interaction, custom_id, locale).await
            }
        }
    }
    .instrument(span)
    .await
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ComponentCustomId {
    Code(code::CustomId),
    Done(done::CustomId),
}

impl Display for ComponentCustomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf))
            .expect("failed to serialize custom_id to MessagePack");
        let wrapper = Base64Display::new(&buf, &STANDARD);
        write!(f, "{}", wrapper)
    }
}

#[derive(Debug, Error)]
pub enum CustomIdError {
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    #[error(transparent)]
    MessagePack(#[from] rmp_serde::decode::Error),
}

impl FromStr for ComponentCustomId {
    type Err = CustomIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = STANDARD.decode(s)?;
        let bytes = buf.as_slice();
        Ok(Self::deserialize(&mut Deserializer::new(bytes))?)
    }
}

impl Into<Option<String>> for ComponentCustomId {
    fn into(self) -> Option<String> {
        Some(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn serde_done() {
        let original = ComponentCustomId::Done(done::CustomId {
            code: "code".into(),
            generated: datetime!(2023-06-18 15:35:34 UTC),
            username: "username".into(),
        });

        let serialized = original.to_string();
        let deserialized: ComponentCustomId = serialized.parse().unwrap();

        assert_eq!(original, deserialized);
    }
}
