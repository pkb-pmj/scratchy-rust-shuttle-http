use serde::Deserialize;
use time::OffsetDateTime;
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::locales::{Locale, ToLocaleEmbedField, ToLocaleString};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct User {
    pub username: String,
    pub id: Option<i64>,
    pub sys_id: i64,
    #[serde(with = "time::serde::iso8601")]
    pub joined: OffsetDateTime,
    pub country: Option<String>,
    pub bio: Option<String>,
    pub work: Option<String>,
    pub status: Option<String>,
    pub school: Option<i64>,
    pub statistics: Option<Statistics>,
}

impl User {
    pub fn url(username: &str) -> String {
        format!("https://scratchdb.lefty.one/v3/user/info/{username}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Statistics {
    pub ranks: Ranks,
    pub loves: Option<i64>,
    pub favorites: Option<i64>,
    pub comments: Option<i64>,
    pub views: Option<i64>,
    pub followers: i64,
    pub following: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Ranks {
    pub country: Country,
    pub loves: i64,
    pub favorites: i64,
    pub comments: i64,
    pub views: i64,
    pub followers: i64,
    pub following: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Country {
    pub loves: i64,
    pub favorites: i64,
    pub comments: i64,
    pub views: i64,
    pub followers: i64,
    pub following: i64,
}

impl ToLocaleString for Statistics {
    fn to_locale_string(&self, locale: Locale) -> String {
        let mut vec = Vec::with_capacity(6);

        if let Some(loves) = self.loves {
            vec.push(locale.user_stats_loves(&loves.to_string()));
        }
        if let Some(favorites) = self.favorites {
            vec.push(locale.user_stats_favorites(&favorites.to_string()));
        }
        if let Some(comments) = self.comments {
            vec.push(locale.user_stats_comments(&comments.to_string()));
        }
        if let Some(views) = self.views {
            vec.push(locale.user_stats_views(&views.to_string()));
        }

        vec.push(locale.user_stats_followers(&self.followers.to_string()));
        vec.push(locale.user_stats_following(&self.following.to_string()));

        vec.join("\n")
    }
}

impl ToLocaleEmbedField for Statistics {
    fn to_locale_embed_field(&self, locale: Locale) -> EmbedFieldBuilder {
        EmbedFieldBuilder::new(locale.user_stats(), self.to_locale_string(locale))
    }
}

impl ToLocaleString for Ranks {
    fn to_locale_string(&self, locale: Locale) -> String {
        vec![
            locale.user_stats_loves(&self.loves.to_string()),
            locale.user_stats_favorites(&self.favorites.to_string()),
            locale.user_stats_comments(&self.comments.to_string()),
            locale.user_stats_views(&self.views.to_string()),
            locale.user_stats_followers(&self.followers.to_string()),
            locale.user_stats_following(&self.following.to_string()),
        ]
        .join("\n")
    }
}

impl ToLocaleEmbedField for Ranks {
    fn to_locale_embed_field(&self, locale: Locale) -> EmbedFieldBuilder {
        EmbedFieldBuilder::new(locale.user_stats_ranks(), self.to_locale_string(locale))
    }
}

impl ToLocaleString for Country {
    fn to_locale_string(&self, locale: Locale) -> String {
        vec![
            locale.user_stats_loves(&self.loves.to_string()),
            locale.user_stats_favorites(&self.favorites.to_string()),
            locale.user_stats_comments(&self.comments.to_string()),
            locale.user_stats_views(&self.views.to_string()),
            locale.user_stats_followers(&self.followers.to_string()),
            locale.user_stats_following(&self.following.to_string()),
        ]
        .join("\n")
    }
}

impl ToLocaleEmbedField for Country {
    fn to_locale_embed_field(&self, locale: Locale) -> EmbedFieldBuilder {
        EmbedFieldBuilder::new(
            locale.user_stats_ranks_country(),
            self.to_locale_string(locale),
        )
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn scratcher() {
        let str = r#"{
            "username": "PMJ_Studio",
            "id": 42178181,
            "sys_id": 50177,
            "joined": "2019-03-11T00:00:00.000Z",
            "country": "Poland",
            "bio": "♂ • † • F4F✖ • Polski • English<br>998 śledzi (wow)<br><br>Chyba już nic tutaj nie robię :/<br>Może kiedyś :)",
            "work": "Głownie projekty długoterminowe i serie:<br>Skoki Narciarskie 5 (Ski Jumping 5)<br>Flight Simulator 3D<br>Starship Simulator 3<br>Nowa Mapa Imperiów<br><br>Polecam:<br> <a href=\"/users/PMJ_MJBCS27\">@PMJ_MJBCS27</a><br> <a href=\"/users/PMJ_JPB14\">@PMJ_JPB14</a>",
            "status": "Scratcher",
            "school": null,
            "statistics": {
                "ranks": {
                    "country": {
                        "loves": 19,
                        "favorites": 18,
                        "comments": 10,
                        "views": 24,
                        "followers": 15,
                        "following": 464
                    },
                    "loves": 2606,
                    "favorites": 2356,
                    "comments": 755,
                    "views": 3611,
                    "followers": 5455,
                    "following": 100632
                },
                "loves": 6727,
                "favorites": 5783,
                "comments": 11745,
                "views": 138406,
                "followers": 1000,
                "following": 127
            }
        }"#;

        let expected = User {
            username: "PMJ_Studio".into(),
            id: Some(42178181),
            sys_id: 50177,
            joined: datetime!(2019-03-11 0:0:00.000 UTC),
            country: Some("Poland".into()),
            bio: Some("♂ • † • F4F✖ • Polski • English<br>998 śledzi (wow)<br><br>Chyba już nic tutaj nie robię :/<br>Może kiedyś :)".into()),
            work: Some("Głownie projekty długoterminowe i serie:<br>Skoki Narciarskie 5 (Ski Jumping 5)<br>Flight Simulator 3D<br>Starship Simulator 3<br>Nowa Mapa Imperiów<br><br>Polecam:<br> <a href=\"/users/PMJ_MJBCS27\">@PMJ_MJBCS27</a><br> <a href=\"/users/PMJ_JPB14\">@PMJ_JPB14</a>".into()),
            status: Some("Scratcher".into()),
            school: None,
            statistics: Some(Statistics {
                ranks: Ranks {
                    country: Country {
                        loves: 19,
                        favorites: 18,
                        comments: 10,
                        views: 24,
                        followers: 15,
                        following: 464
                    },
                    loves: 2606,
                    favorites: 2356,
                    comments: 755,
                    views: 3611,
                    followers: 5455,
                    following: 100632
                },
                loves: Some(6727),
                favorites: Some(5783),
                comments: Some(11745),
                views: Some(138406),
                followers: 1000,
                following: 127
            })
        };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn new_scratcher() {
        let str = r#"{
            "username": "MW--0",
            "id": 79448740,
            "sys_id": 10084346,
            "joined": "2021-07-17T00:00:00.000Z",
            "country": "Poland",
            "bio": "Konto do nagradzania zwycięzców Miesiąca Wyzwań.",
            "work": "Miesiąc Wyzwań -  <a href=\"/users/norbert00\">@norbert00</a><br><a href=\"https://scratch.mit.edu/studios/26661367/\">https://scratch.mit.edu/studios/26661367/</a><br><br>Miesiąc Wyzwań 2 -  <a href=\"/users/PMJ_Studio\">@PMJ_Studio</a><br><a href=\"https://scratch.mit.edu/studios/29864749/\">https://scratch.mit.edu/studios/29864749/</a>",
            "status": "New Scratcher",
            "school": null
        }"#;

        let expected = User {
            username: "MW--0".into(),
            id: Some(79448740),
            sys_id: 10084346,
            joined: datetime!(2021-07-17 00:00:00.000 UTC),
            country: Some("Poland".into()),
            bio: Some("Konto do nagradzania zwycięzców Miesiąca Wyzwań.".into()),
            work: Some("Miesiąc Wyzwań -  <a href=\"/users/norbert00\">@norbert00</a><br><a href=\"https://scratch.mit.edu/studios/26661367/\">https://scratch.mit.edu/studios/26661367/</a><br><br>Miesiąc Wyzwań 2 -  <a href=\"/users/PMJ_Studio\">@PMJ_Studio</a><br><a href=\"https://scratch.mit.edu/studios/29864749/\">https://scratch.mit.edu/studios/29864749/</a>".into()),
            status: Some("New Scratcher".into()),
            school: None,
            statistics: None,
        };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn student() {
        let str = r#"{
            "username": "Kus4bMa",
            "id": 71174519,
            "sys_id": 7204831,
            "joined": "2021-02-01T00:00:00.000Z",
            "country": "Poland",
            "bio": "",
            "work": "",
            "status": "New Scratcher",
            "school": 381205
        }"#;

        let expected = User {
            username: "Kus4bMa".into(),
            id: Some(71174519),
            sys_id: 7204831,
            joined: datetime!(2021-02-01 00:00:00.000 UTC),
            country: Some("Poland".into()),
            bio: Some("".into()),
            work: Some("".into()),
            status: Some("New Scratcher".into()),
            school: Some(381205),
            statistics: None,
        };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn teacher() {
        let str = r#"{
            "username": "BELFER2020",
            "id": 60292946,
            "sys_id": 7204830,
            "joined": "2020-05-09T00:00:00.000Z",
            "country": "Poland",
            "bio": "",
            "work": "",
            "status": "Teacher Account",
            "school": null,
            "statistics": {
                "ranks": {
                    "country": {
                        "loves": 3683,
                        "favorites": 3734,
                        "comments": 2072,
                        "views": 3302,
                        "followers": 16,
                        "following": 32
                    },
                    "loves": 543452,
                    "favorites": 563958,
                    "comments": 342021,
                    "views": 469361,
                    "followers": 5564,
                    "following": 7068
                },
                "loves": 4,
                "favorites": 1,
                "comments": 0,
                "views": 127,
                "followers": 987,
                "following": 981
            }
        }"#;

        let expected = User {
            username: "BELFER2020".into(),
            id: Some(60292946),
            sys_id: 7204830,
            joined: datetime!(2020-05-09 00:00:00.000 UTC),
            country: Some("Poland".into()),
            bio: Some("".into()),
            work: Some("".into()),
            status: Some("Teacher Account".into()),
            school: None,
            statistics: Some(Statistics {
                ranks: Ranks {
                    country: Country {
                        loves: 3683,
                        favorites: 3734,
                        comments: 2072,
                        views: 3302,
                        followers: 16,
                        following: 32,
                    },
                    loves: 543452,
                    favorites: 563958,
                    comments: 342021,
                    views: 469361,
                    followers: 5564,
                    following: 7068,
                },
                loves: Some(4),
                favorites: Some(1),
                comments: Some(0),
                views: Some(127),
                followers: 987,
                following: 981,
            }),
        };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn gdpr() {
        let str = r#"{
            "username": "gdpr0000001",
            "id": null,
            "sys_id": 4506,
            "joined": "1970-01-01T00:00:00.000Z",
            "country": null,
            "bio": null,
            "work": null,
            "status": null,
            "school": null,
            "statistics": {
                "ranks": {
                    "country": {
                        "loves": 1,
                        "favorites": 1,
                        "comments": 1,
                        "views": 1,
                        "followers": 1,
                        "following": 3752
                    },
                    "loves": 1,
                    "favorites": 1,
                    "comments": 1,
                    "views": 1,
                    "followers": 455,
                    "following": 723757
                },
                "followers": 4326,
                "following": 0
            }
        }"#;

        let expected = User {
            username: "gdpr0000001".into(),
            id: None,
            sys_id: 4506,
            joined: datetime!(1970-01-01 00:00:00.000 UTC),
            country: None,
            bio: None,
            work: None,
            status: None,
            school: None,
            statistics: Some(Statistics {
                ranks: Ranks {
                    country: Country {
                        loves: 1,
                        favorites: 1,
                        comments: 1,
                        views: 1,
                        followers: 1,
                        following: 3752,
                    },
                    loves: 1,
                    favorites: 1,
                    comments: 1,
                    views: 1,
                    followers: 455,
                    following: 723757,
                },
                followers: 4326,
                following: 0,
                loves: None,
                favorites: None,
                comments: None,
                views: None,
            }),
        };

        let actual: User = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }
}
