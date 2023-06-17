use serde::Deserialize;
use time::OffsetDateTime;

use crate::scratch::Url;

impl Url for Vec<Comment> {
    type UrlArgs = i64;

    fn url(studio_id: Self::UrlArgs) -> String {
        format!("https://api.scratch.mit.edu/users/{studio_id}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub commentee_id: Option<i64>,
    pub content: String,
    #[serde(with = "time::serde::iso8601")]
    pub datetime_created: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub datetime_modified: OffsetDateTime,
    pub visibility: String,
    pub author: Author,
    pub reply_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Author {
    pub id: i64,
    pub username: String,
    pub scratchteam: bool,
    pub image: String,
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn basic() {
        let str = r#"[
            {
                "id": 225945888,
                "parent_id": null,
                "commentee_id": null,
                "content": "N3f4g4L3r6i2A4c3S5t5",
                "datetime_created": "2023-06-08T16:28:30.000Z",
                "datetime_modified": "2023-06-08T16:28:30.000Z",
                "visibility": "visible",
                "author": {
                    "id": 106748322,
                    "username": "Patyczakowy_Mapper",
                    "scratchteam": false,
                    "image": "https://cdn2.scratch.mit.edu/get_image/user/106748322_60x60.png"
                },
                "reply_count": 0
            },
            {
                "id": 225022450,
                "parent_id": null,
                "commentee_id": null,
                "content": "a5N9R6N0e1d2d8y8p3T1",
                "datetime_created": "2023-05-29T16:20:56.000Z",
                "datetime_modified": "2023-05-29T16:20:56.000Z",
                "visibility": "visible",
                "author": {
                    "id": 90746635,
                    "username": "Dragonoidowy",
                    "scratchteam": false,
                    "image": "https://cdn2.scratch.mit.edu/get_image/user/90746635_60x60.png"
                },
                "reply_count": 0
            }
        ]"#;

        let expected = vec![
            Comment {
                id: 225945888,
                parent_id: None,
                commentee_id: None,
                content: "N3f4g4L3r6i2A4c3S5t5".into(),
                datetime_created: datetime!(2023-06-08 16:28:30.000 UTC),
                datetime_modified: datetime!(2023-06-08 16:28:30.000 UTC),
                visibility: "visible".into(),
                author: Author {
                    id: 106748322,
                    username: "Patyczakowy_Mapper".into(),
                    scratchteam: false,
                    image: "https://cdn2.scratch.mit.edu/get_image/user/106748322_60x60.png".into(),
                },
                reply_count: 0,
            },
            Comment {
                id: 225022450,
                parent_id: None,
                commentee_id: None,
                content: "a5N9R6N0e1d2d8y8p3T1".into(),
                datetime_created: datetime!(2023-05-29 16:20:56.000 UTC),
                datetime_modified: datetime!(2023-05-29 16:20:56.000 UTC),
                visibility: "visible".into(),
                author: Author {
                    id: 90746635,
                    username: "Dragonoidowy".into(),
                    scratchteam: false,
                    image: "https://cdn2.scratch.mit.edu/get_image/user/90746635_60x60.png".into(),
                },
                reply_count: 0,
            },
        ];

        let actual: Vec<Comment> = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }
}
