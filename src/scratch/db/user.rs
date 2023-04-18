use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub id: Option<i64>,
    pub sys_id: i64,
    pub joined: String,
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

#[derive(Debug, Clone, Deserialize)]
pub struct Statistics {
    pub ranks: Ranks,
    pub loves: i64,
    pub favorites: i64,
    pub comments: i64,
    pub views: i64,
    pub followers: i64,
    pub following: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ranks {
    pub country: Country,
    pub loves: i64,
    pub favorites: i64,
    pub comments: i64,
    pub views: i64,
    pub followers: i64,
    pub following: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Country {
    pub loves: i64,
    pub favorites: i64,
    pub comments: i64,
    pub views: i64,
    pub followers: i64,
    pub following: i64,
}
