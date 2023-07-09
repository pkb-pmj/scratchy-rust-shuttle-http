use async_trait::async_trait;
use hyper::header::AUTHORIZATION;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use reqwest::Client;

use crate::state::Config;

use super::{
    metadata::RoleConnectionData,
    model::{Metadata, RoleConnection},
};

pub fn create_oauth_client(config: &Config) -> BasicClient {
    let client_id = ClientId::new(config.client_id.to_owned());
    let client_secret = ClientSecret::new(config.client_secret.to_owned());
    let auth_url = AuthUrl::new("https://discord.com/api/oauth2/authorize".into()).unwrap();
    let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".into()).unwrap();
    let redirect_url = RedirectUrl::from_url(config.redirect_url.to_owned());

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url)
}

#[async_trait]
pub trait RoleConnectionClient {
    type Error;
    type Data;

    async fn get_metadata(
        &self,
        client_id: &str,
        token: &str,
    ) -> Result<Vec<Metadata>, Self::Error>;

    async fn put_metadata(
        &self,
        client_id: &str,
        token: &str,
        metadata: Vec<Metadata>,
    ) -> Result<Vec<Metadata>, Self::Error>;

    async fn get_role_connection(
        &self,
        client_id: &str,
        token: &str,
    ) -> Result<RoleConnection<Self::Data>, Self::Error>;

    async fn put_role_connection(
        &self,
        client_id: &str,
        token: &str,
        data: &RoleConnection<Self::Data>,
    ) -> Result<RoleConnection<Self::Data>, Self::Error>;
}

fn metadata_url(client_id: &str) -> String {
    format!("https://discord.com/api/v10/applications/{client_id}/role-connections/metadata")
}

fn role_connection_url(client_id: &str) -> String {
    format!("https://discord.com/api/v10/users/@me/applications/{client_id}/role-connection")
}

#[async_trait]
impl RoleConnectionClient for Client {
    type Error = reqwest::Error;
    type Data = RoleConnectionData;

    async fn get_metadata(
        &self,
        client_id: &str,
        token: &str,
    ) -> Result<Vec<Metadata>, Self::Error> {
        self.get(&metadata_url(client_id))
            .header(AUTHORIZATION, &format!("Bot {}", token))
            .send()
            .await?
            .json()
            .await
    }

    async fn put_metadata(
        &self,
        client_id: &str,
        token: &str,
        metadata: Vec<Metadata>,
    ) -> Result<Vec<Metadata>, Self::Error> {
        self.put(&metadata_url(client_id))
            .header(AUTHORIZATION, &format!("Bot {}", token))
            .json(&metadata)
            .send()
            .await?
            .json()
            .await
    }

    async fn get_role_connection(
        &self,
        client_id: &str,
        token: &str,
    ) -> Result<RoleConnection<Self::Data>, Self::Error> {
        self.get(&role_connection_url(client_id))
            .bearer_auth(token)
            .send()
            .await?
            .json()
            .await
    }

    async fn put_role_connection(
        &self,
        client_id: &str,
        token: &str,
        data: &RoleConnection<Self::Data>,
    ) -> Result<RoleConnection<Self::Data>, Self::Error> {
        self.put(&role_connection_url(client_id))
            .bearer_auth(token)
            .json(&data)
            .send()
            .await?
            .json()
            .await
    }
}
