use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use crate::state::Config;

pub fn create_oauth_client(config: &Config) -> BasicClient {
    let client_id = ClientId::new(config.client_id.to_owned());
    let client_secret = ClientSecret::new(config.client_secret.to_owned());
    let auth_url = AuthUrl::new("https://discord.com/api/oauth2/authorize".into()).unwrap();
    let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".into()).unwrap();
    let redirect_url = RedirectUrl::from_url(config.redirect_url.to_owned());

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url)
}
