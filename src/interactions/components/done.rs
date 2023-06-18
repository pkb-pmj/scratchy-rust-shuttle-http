use time::OffsetDateTime;
use twilight_model::channel::message::{
    component::{Button, ButtonStyle},
    Component,
};

use crate::locales::Locale;

pub fn build(
    username: String,
    code: String,
    timestamp: OffsetDateTime,
    locale: Locale,
) -> Component {
    let timestamp = timestamp.unix_timestamp();

    Component::Button(Button {
        custom_id: Some(format!("done {username} {code} {timestamp}").into()),
        disabled: false,
        emoji: None,
        label: Some(locale.verify_comment()),
        style: ButtonStyle::Primary,
        url: None,
    })
}
