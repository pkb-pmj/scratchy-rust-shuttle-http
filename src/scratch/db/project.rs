use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: i64,
    pub sys_id: i64,
    pub username: String,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub times: Times,
    pub remix: Remix,
    pub statistics: Statistics,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Times {
    pub created: String,
    pub modified: String,
    pub shared: String,
    pub last_check: String,
    pub last_metadata_check: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Remix {
    pub parent: Option<i64>,
    pub root: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Statistics {
    pub ranks: Ranks,
    pub views: i64,
    pub loves: i64,
    pub favorites: i64,
    pub comments: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ranks {
    pub views: i64,
    pub loves: i64,
    pub favorites: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub version: i64,
    pub costumes: i64,
    pub blocks: i64,
    pub variables: i64,
    pub assets: i64,
    pub hash: String,
    pub user_agent: String,
    pub history: HashMap<String, String>,
}
