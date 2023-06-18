use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use twilight_model::channel::message::{
    component::{Button, ButtonStyle},
    Component,
};

use crate::locales::Locale;

use super::ComponentCustomId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomId {
    pub username: String,
    pub code: String,
    #[serde(with = "time::serde::iso8601")]
    pub expires: OffsetDateTime,
}

pub fn build(custom_id: CustomId, locale: Locale) -> Component {
    Component::Button(Button {
        custom_id: ComponentCustomId::Done(custom_id).into(),
        disabled: false,
        emoji: None,
        label: Some(locale.verify_comment()),
        style: ButtonStyle::Primary,
        url: None,
    })
}
