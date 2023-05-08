use serde::Deserialize;
use time::OffsetDateTime;

use crate::scratch::Requestable;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub scratchteam: bool,
    pub history: History,
    pub profile: Profile,
}

impl Requestable for User {
    type UrlArgs = String;

    fn url(username: Self::UrlArgs) -> String {
        format!("https://api.scratch.mit.edu/users/{username}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct History {
    #[serde(with = "time::serde::iso8601")]
    pub joined: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub images: Images,
    pub status: String,
    pub bio: String,
    pub country: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Images {
    #[serde(rename = "90x90")]
    pub n90x90: String,
    #[serde(rename = "60x60")]
    pub n60x60: String,
    #[serde(rename = "55x55")]
    pub n55x55: String,
    #[serde(rename = "50x50")]
    pub n50x50: String,
    #[serde(rename = "32x32")]
    pub n32x32: String,
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn scratcher() {
        let str = r#"{
            "id": 42178181,
            "username": "PMJ_Studio",
            "scratchteam": false,
            "history": {
                "joined": "2019-03-11T20:54:16.000Z"
            },
            "profile": {
                "id": 41297648,
                "images": {
                    "90x90": "https://cdn2.scratch.mit.edu/get_image/user/42178181_90x90.png?v=",
                    "60x60": "https://cdn2.scratch.mit.edu/get_image/user/42178181_60x60.png?v=",
                    "55x55": "https://cdn2.scratch.mit.edu/get_image/user/42178181_55x55.png?v=",
                    "50x50": "https://cdn2.scratch.mit.edu/get_image/user/42178181_50x50.png?v=",
                    "32x32": "https://cdn2.scratch.mit.edu/get_image/user/42178181_32x32.png?v="
                },
                "status": "Nothing - my Scratch journey is finished. Now I'm mostly using JavaScript, Rust, C++ and Python, doing mostly webdev, embedded and entering the AI world.\n\nBrothers:\n@PMJ_MJBCS27\n@PMJ_JPB14",
                "bio": "♂ • F4F✖ • Polski • English\nTrust me, if you're not Polish, use English because it will be much easier to understand.\nWho would have thought I would get all the way to 1000 followers :D",
                "country": "Poland"
            }
        }"#;

        let expected = User {
            id: 42178181,
            username: "PMJ_Studio".into(),
            scratchteam: false,
            history: History {
              joined: datetime!(2019-03-11 20:54:16.000 UTC),
            },
            profile: Profile {
              id: 41297648,
              images: Images {
                n90x90: "https://cdn2.scratch.mit.edu/get_image/user/42178181_90x90.png?v=".into(),
                n60x60: "https://cdn2.scratch.mit.edu/get_image/user/42178181_60x60.png?v=".into(),
                n55x55: "https://cdn2.scratch.mit.edu/get_image/user/42178181_55x55.png?v=".into(),
                n50x50: "https://cdn2.scratch.mit.edu/get_image/user/42178181_50x50.png?v=".into(),
                n32x32: "https://cdn2.scratch.mit.edu/get_image/user/42178181_32x32.png?v=".into()
              },
              status: "Nothing - my Scratch journey is finished. Now I'm mostly using JavaScript, Rust, C++ and Python, doing mostly webdev, embedded and entering the AI world.\n\nBrothers:\n@PMJ_MJBCS27\n@PMJ_JPB14".into(),
              bio: "♂ • F4F✖ • Polski • English\nTrust me, if you're not Polish, use English because it will be much easier to understand.\nWho would have thought I would get all the way to 1000 followers :D".into(),
              country: Some("Poland".into())
            }
          };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn new_scratcher() {
        let str = r#"{
            "id": 79448740,
            "username": "MW--0",
            "scratchteam": false,
            "history": {
                "joined": "2021-07-17T18:21:53.000Z"
            },
            "profile": {
                "id": 78487416,
                "images": {
                    "90x90": "https://cdn2.scratch.mit.edu/get_image/user/79448740_90x90.png?v=",
                    "60x60": "https://cdn2.scratch.mit.edu/get_image/user/79448740_60x60.png?v=",
                    "55x55": "https://cdn2.scratch.mit.edu/get_image/user/79448740_55x55.png?v=",
                    "50x50": "https://cdn2.scratch.mit.edu/get_image/user/79448740_50x50.png?v=",
                    "32x32": "https://cdn2.scratch.mit.edu/get_image/user/79448740_32x32.png?v="
                },
                "status": "Miesiąc Wyzwań - @norbert00\nhttps://scratch.mit.edu/studios/26661367/\n\nMiesiąc Wyzwań 2 - @PMJ_Studio\nhttps://scratch.mit.edu/studios/29864749/",
                "bio": "Konto do nagradzania zwycięzców Miesiąca Wyzwań.",
                "country": "Poland"
            }
        }"#;

        let expected = User {
            id: 79448740,
            username: "MW--0".into(),
            scratchteam: false,
            history: History {
              joined: datetime!(2021-07-17 18:21:53.000 UTC),
            },
            profile: Profile {
              id: 78487416,
              images: Images {
                n90x90: "https://cdn2.scratch.mit.edu/get_image/user/79448740_90x90.png?v=".into(),
                n60x60: "https://cdn2.scratch.mit.edu/get_image/user/79448740_60x60.png?v=".into(),
                n55x55: "https://cdn2.scratch.mit.edu/get_image/user/79448740_55x55.png?v=".into(),
                n50x50: "https://cdn2.scratch.mit.edu/get_image/user/79448740_50x50.png?v=".into(),
                n32x32: "https://cdn2.scratch.mit.edu/get_image/user/79448740_32x32.png?v=".into()
              },
              status: "Miesiąc Wyzwań - @norbert00\nhttps://scratch.mit.edu/studios/26661367/\n\nMiesiąc Wyzwań 2 - @PMJ_Studio\nhttps://scratch.mit.edu/studios/29864749/".into(),
              bio: "Konto do nagradzania zwycięzców Miesiąca Wyzwań.".into(),
              country: Some("Poland".into())
            }
          };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn student() {
        let str = r#"{
            "id": 71174519,
            "username": "Kus4bMa",
            "scratchteam": false,
            "history": {
                "joined": "2021-02-01T20:00:58.000Z"
            },
            "profile": {
                "id": 70229113,
                "images": {
                    "90x90": "https://cdn2.scratch.mit.edu/get_image/user/71174519_90x90.png?v=",
                    "60x60": "https://cdn2.scratch.mit.edu/get_image/user/71174519_60x60.png?v=",
                    "55x55": "https://cdn2.scratch.mit.edu/get_image/user/71174519_55x55.png?v=",
                    "50x50": "https://cdn2.scratch.mit.edu/get_image/user/71174519_50x50.png?v=",
                    "32x32": "https://cdn2.scratch.mit.edu/get_image/user/71174519_32x32.png?v="
                },
                "status": "",
                "bio": "",
                "country": "Poland"
            }
        }"#;

        let expected = User {
            id: 71174519,
            username: "Kus4bMa".into(),
            scratchteam: false,
            history: History {
                joined: datetime!(2021-02-01 20:00:58.000 UTC),
            },
            profile: Profile {
                id: 70229113,
                images: Images {
                    n90x90: "https://cdn2.scratch.mit.edu/get_image/user/71174519_90x90.png?v="
                        .into(),
                    n60x60: "https://cdn2.scratch.mit.edu/get_image/user/71174519_60x60.png?v="
                        .into(),
                    n55x55: "https://cdn2.scratch.mit.edu/get_image/user/71174519_55x55.png?v="
                        .into(),
                    n50x50: "https://cdn2.scratch.mit.edu/get_image/user/71174519_50x50.png?v="
                        .into(),
                    n32x32: "https://cdn2.scratch.mit.edu/get_image/user/71174519_32x32.png?v="
                        .into(),
                },
                status: "".into(),
                bio: "".into(),
                country: Some("Poland".into()),
            },
        };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn teacher() {
        let str = r#"{
            "id": 60292946,
            "username": "BELFER2020",
            "scratchteam": false,
            "history": {
                "joined": "2020-05-09T07:41:24.000Z"
            },
            "profile": {
                "id": 59363344,
                "images": {
                    "90x90": "https://cdn2.scratch.mit.edu/get_image/user/60292946_90x90.png?v=",
                    "60x60": "https://cdn2.scratch.mit.edu/get_image/user/60292946_60x60.png?v=",
                    "55x55": "https://cdn2.scratch.mit.edu/get_image/user/60292946_55x55.png?v=",
                    "50x50": "https://cdn2.scratch.mit.edu/get_image/user/60292946_50x50.png?v=",
                    "32x32": "https://cdn2.scratch.mit.edu/get_image/user/60292946_32x32.png?v="
                },
                "status": "",
                "bio": "",
                "country": "Poland"
            }
        }"#;

        let expected = User {
            id: 60292946,
            username: "BELFER2020".into(),
            scratchteam: false,
            history: History {
                joined: datetime!(2020-05-09 07:41:24.000 UTC),
            },
            profile: Profile {
                id: 59363344,
                images: Images {
                    n90x90: "https://cdn2.scratch.mit.edu/get_image/user/60292946_90x90.png?v="
                        .into(),
                    n60x60: "https://cdn2.scratch.mit.edu/get_image/user/60292946_60x60.png?v="
                        .into(),
                    n55x55: "https://cdn2.scratch.mit.edu/get_image/user/60292946_55x55.png?v="
                        .into(),
                    n50x50: "https://cdn2.scratch.mit.edu/get_image/user/60292946_50x50.png?v="
                        .into(),
                    n32x32: "https://cdn2.scratch.mit.edu/get_image/user/60292946_32x32.png?v="
                        .into(),
                },
                status: "".into(),
                bio: "".into(),
                country: Some("Poland".into()),
            },
        };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn gdpr() {
        let str = r#"{
            "id": 15680061,
            "username": "gdpr0000001",
            "scratchteam": false,
            "history": {
                "joined": "2000-01-01T00:00:00.000Z"
            },
            "profile": {
                "id": 14844877,
                "images": {
                    "90x90": "https://cdn2.scratch.mit.edu/get_image/user/15680061_90x90.png?v=",
                    "60x60": "https://cdn2.scratch.mit.edu/get_image/user/15680061_60x60.png?v=",
                    "55x55": "https://cdn2.scratch.mit.edu/get_image/user/15680061_55x55.png?v=",
                    "50x50": "https://cdn2.scratch.mit.edu/get_image/user/15680061_50x50.png?v=",
                    "32x32": "https://cdn2.scratch.mit.edu/get_image/user/15680061_32x32.png?v="
                },
                "status": "",
                "bio": "",
                "country": null
            }
        }"#;

        let expected = User {
            id: 15680061,
            username: "gdpr0000001".into(),
            scratchteam: false,
            history: History {
                joined: datetime!(2000-01-01 00:00:00.000 UTC),
            },
            profile: Profile {
                id: 14844877,
                images: Images {
                    n90x90: "https://cdn2.scratch.mit.edu/get_image/user/15680061_90x90.png?v="
                        .into(),
                    n60x60: "https://cdn2.scratch.mit.edu/get_image/user/15680061_60x60.png?v="
                        .into(),
                    n55x55: "https://cdn2.scratch.mit.edu/get_image/user/15680061_55x55.png?v="
                        .into(),
                    n50x50: "https://cdn2.scratch.mit.edu/get_image/user/15680061_50x50.png?v="
                        .into(),
                    n32x32: "https://cdn2.scratch.mit.edu/get_image/user/15680061_32x32.png?v="
                        .into(),
                },
                status: "".into(),
                bio: "".into(),
                country: None,
            },
        };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }
}
