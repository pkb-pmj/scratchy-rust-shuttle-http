use serde::Deserialize;

use super::user;

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub author: Author,
    pub image: String,
    pub images: Images,
    pub history: History,
    pub stats: Stats,
    pub remix: Remix,
    pub project_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub id: i64,
    pub username: String,
    pub scratchteam: bool,
    pub history: user::History,
    pub profile: Profile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub id: Option<i64>,
    pub images: user::Images,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Images {
    #[serde(rename = "282x218")]
    pub n282x218: String,
    #[serde(rename = "216x163")]
    pub n216x163: String,
    #[serde(rename = "200x200")]
    pub n200x200: String,
    #[serde(rename = "144x108")]
    pub n144x108: String,
    #[serde(rename = "135x102")]
    pub n135x102: String,
    #[serde(rename = "100x80")]
    pub n100x80: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct History {
    pub created: String,
    pub modified: String,
    pub shared: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stats {
    pub views: i64,
    pub loves: i64,
    pub favorites: i64,
    pub remixes: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Remix {
    pub parent: Option<i64>,
    pub root: Option<i64>,
}
