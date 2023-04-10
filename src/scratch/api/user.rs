use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub scratchteam: bool,
    pub history: History,
    pub profile: Profile,
}

impl User {
    pub fn url(username: &str) -> String {
        format!("https://api.scratch.mit.edu/users/{username}")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct History {
    pub joined: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub images: Images,
    pub status: String,
    pub bio: String,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
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
