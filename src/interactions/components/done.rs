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
    database::{link_account, Database, LinkError},
    interactions::{context::MessageComponentInteraction, InteractionError},
    linked_roles::RoleConnectionUpdater,
    locales::Locale,
    scratch::{
        api::{studio::Comment, ScratchAPIClient},
        site::user_link,
        STUDIO_ID,
    },
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

    let comments = state
        .reqwest_client
        .get_scratch_api_studio_comments(STUDIO_ID)
        .await?
        // Assume the studio hasn't been deleted
        .unwrap();

    if let Err(err) = validate_comment(comments, custom_id.to_owned()) {
        let message = match err {
            ValidateCommentError::CommentNotFound => locale.comment_not_found(),
            ValidateCommentError::InvalidAccount(actual) => {
                locale.wrong_account(&user_link(&actual), &user_link(&custom_id.username))
            }
            ValidateCommentError::InvalidCode(_) => locale.invalid_code(),
        };

        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(message)
                    .build(),
            ),
        });
    };

    if let Err(err) = link_account(&state.pool, custom_id.username.to_owned(), author_id).await? {
        let message = match err {
            LinkError::AlreadyLinkedToYou => {
                locale.already_linked_to_you(&user_link(&custom_id.username))
            }
            LinkError::AlreadyLinkedToOther(id) => locale.already_linked_to_other(
                &id.mention().to_string(),
                &user_link(&custom_id.username),
            ),
        };

        return Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(message)
                    .allowed_mentions(Default::default())
                    .build(),
            ),
        });
    }

    if state.pool.get_token(author_id).await?.is_some() {
        state.update_role_connection(author_id).await.unwrap();
    }

    let message = format!(
        "{}\n\n{}",
        locale.successfully_linked(
            &author_id.mention().to_string(),
            &user_link(&custom_id.username),
        ),
        locale.linked_roles_message(),
    );

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
enum ValidateCommentError {
    InvalidAccount(String),
    InvalidCode(String),
    CommentNotFound,
}

fn validate_comment(
    comments: Vec<Comment>,
    custom_id: CustomId,
) -> Result<(), ValidateCommentError> {
    let comments: Vec<_> = comments
        .into_iter()
        .filter(|comment| comment.datetime_created > custom_id.generated)
        .collect();

    let valid_code = |comment: &&Comment| comment.content.trim() == custom_id.code;
    let valid_username = |comment: &&Comment| {
        comment.author.username.to_lowercase() == custom_id.username.to_lowercase()
    };

    if let Some(_) = comments.iter().filter(valid_code).find(valid_username) {
        Ok(())
    } else {
        Err(if let Some(comment) = comments.iter().find(valid_code) {
            ValidateCommentError::InvalidAccount(comment.author.username.to_string())
        } else if let Some(comment) = comments.iter().find(valid_username) {
            ValidateCommentError::InvalidCode(comment.content.to_string())
        } else {
            ValidateCommentError::CommentNotFound
        })
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

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn comment_not_found() {
        let comments = comments(&[("code2", "username2"), ("code3", "username3")]);

        let result = validate_comment(comments, custom_id());

        assert_eq!(result, Err(ValidateCommentError::CommentNotFound));
    }

    #[test]
    fn invalid_code() {
        let comments = comments(&[
            ("code2", "username1"),
            ("code2", "username2"),
            ("code3", "username3"),
        ]);

        let result = validate_comment(comments, custom_id());

        assert_eq!(
            result,
            Err(ValidateCommentError::InvalidCode("code2".into()))
        );
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
            Err(ValidateCommentError::InvalidAccount("username2".into()))
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
            Err(ValidateCommentError::InvalidAccount("username2".into()))
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

        assert_eq!(result, Err(ValidateCommentError::CommentNotFound));
    }
}
