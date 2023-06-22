use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use twilight_mention::Mention;
use twilight_model::{
    channel::message::{
        component::{Button, ButtonStyle},
        Component,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{
    database::{link_account, LinkResult},
    interactions::{context::MessageComponentInteraction, InteractionError},
    locales::Locale,
    scratch::{api::studio::Comment, site::user_link, STUDIO_ID},
    state::AppState,
};

use super::ComponentCustomId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomId {
    pub username: String,
    pub code: String,
    #[serde(with = "time::serde::iso8601")]
    pub generated: OffsetDateTime,
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

pub async fn run(
    state: AppState,
    interaction: MessageComponentInteraction,
    custom_id: CustomId,
    locale: Locale,
) -> Result<InteractionResponse, InteractionError> {
    let author_id = interaction.author_id().unwrap();
    let expires = custom_id.generated.saturating_add(Duration::minutes(5));

    if OffsetDateTime::now_utc() > expires {
        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(locale.code_expired())
                    .build(),
            ),
        });
    }

    let comments = state.client.get::<Vec<Comment>>(STUDIO_ID).await.unwrap();

    let err = match validate_comment(comments, custom_id.to_owned()) {
        ValidateCommentResult::CommentNotFound => Some(locale.comment_not_found()),
        ValidateCommentResult::InvalidAccount(actual) => {
            Some(locale.wrong_account(&user_link(&actual), &user_link(&custom_id.username)))
        }
        ValidateCommentResult::InvalidCode(_) => Some(locale.invalid_code()),
        ValidateCommentResult::Ok => None,
    };

    if let Some(message) = err {
        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(message)
                    .build(),
            ),
        });
    }

    let message = match link_account(&state.pool, custom_id.username.to_owned(), author_id)
        .await
        .unwrap()
    {
        LinkResult::AlreadyLinkedToYou => {
            locale.already_linked_to_you(&user_link(&custom_id.username))
        }
        LinkResult::AlreadyLinkedToOther(id) => locale
            .already_linked_to_other(&id.mention().to_string(), &user_link(&custom_id.username)),
        LinkResult::SuccessfullyLinked => locale.successfully_linked(
            &author_id.mention().to_string(),
            &user_link(&custom_id.username),
        ),
    };

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content(message)
                .allowed_mentions(Default::default())
                .build(),
        ),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValidateCommentResult {
    Ok,
    InvalidAccount(String),
    InvalidCode(String),
    CommentNotFound,
}

fn validate_comment(comments: Vec<Comment>, custom_id: CustomId) -> ValidateCommentResult {
    let comments: Vec<_> = comments
        .into_iter()
        .filter(|comment| comment.datetime_created > custom_id.generated)
        .collect();

    let valid_code = |comment: &&Comment| comment.content.trim() == custom_id.code;
    let valid_username = |comment: &&Comment| {
        comment.author.username.to_lowercase() == custom_id.username.to_lowercase()
    };

    if let Some(_) = comments.iter().filter(valid_code).find(valid_username) {
        ValidateCommentResult::Ok
    } else {
        if let Some(comment) = comments.iter().find(valid_code) {
            ValidateCommentResult::InvalidAccount(comment.author.username.to_string())
        } else if let Some(comment) = comments.iter().find(valid_username) {
            ValidateCommentResult::InvalidCode(comment.content.to_string())
        } else {
            ValidateCommentResult::CommentNotFound
        }
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use crate::scratch::api::studio::Author;

    use super::*;

    fn comment(code: &str, username: &str) -> Comment {
        Comment {
            id: 225945888,
            parent_id: None,
            commentee_id: None,
            content: code.into(),
            datetime_created: datetime!(2023-06-08 16:01:00.000 UTC),
            datetime_modified: datetime!(2023-06-08 16:01:00.000 UTC),
            visibility: "visible".into(),
            author: Author {
                id: 106748322,
                username: username.into(),
                scratchteam: false,
                image: "https://cdn2.scratch.mit.edu/get_image/user/106748322_60x60.png".into(),
            },
            reply_count: 0,
        }
    }

    fn comments(data: &[(&str, &str)]) -> Vec<Comment> {
        data.into_iter()
            .map(|(code, username)| comment(code, username))
            .collect()
    }

    fn custom_id() -> CustomId {
        CustomId {
            code: "code1".into(),
            generated: datetime!(2023-06-08 16:00:00.000 UTC),
            username: "username1".into(),
        }
    }

    #[test]
    fn ok() {
        let comments = comments(&[
            ("code1", "username1"),
            ("code2", "username1"),
            ("code1", "username2"),
            ("code2", "username2"),
        ]);

        let result = validate_comment(comments, custom_id());

        assert_eq!(result, ValidateCommentResult::Ok);
    }

    #[test]
    fn comment_not_found() {
        let comments = comments(&[("code2", "username2"), ("code3", "username3")]);

        let result = validate_comment(comments, custom_id());

        assert_eq!(result, ValidateCommentResult::CommentNotFound);
    }

    #[test]
    fn invalid_code() {
        let comments = comments(&[
            ("code2", "username1"),
            ("code2", "username2"),
            ("code3", "username3"),
        ]);

        let result = validate_comment(comments, custom_id());

        assert_eq!(result, ValidateCommentResult::InvalidCode("code2".into()));
    }

    #[test]
    fn invalid_account() {
        let comments = comments(&[
            ("code1", "username2"),
            ("code2", "username2"),
            ("code3", "username3"),
        ]);

        let result = validate_comment(comments, custom_id());

        assert_eq!(
            result,
            ValidateCommentResult::InvalidAccount("username2".into())
        );
    }

    #[test]
    fn invalid_both() {
        let comments = comments(&[
            ("code1", "username2"),
            ("code2", "username1"),
            ("code2", "username2"),
            ("code3", "username3"),
        ]);

        let result = validate_comment(comments, custom_id());

        assert_eq!(
            result,
            ValidateCommentResult::InvalidAccount("username2".into())
        );
    }

    #[test]
    fn too_early() {
        let comments = comments(&[
            ("code1", "username1"),
            ("code2", "username1"),
            ("code1", "username2"),
            ("code2", "username2"),
        ]);

        let custom_id = CustomId {
            code: "code1".into(),
            generated: datetime!(2023-06-08 17:00:00.000 UTC),
            username: "username1".into(),
        };

        let result = validate_comment(comments, custom_id);

        assert_eq!(result, ValidateCommentResult::CommentNotFound);
    }
}
