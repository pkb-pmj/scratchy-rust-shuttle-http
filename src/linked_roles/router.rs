use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::CookieJar;
use hyper::{header, HeaderMap, StatusCode};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use twilight_http::Client as TwilightClient;

use crate::{database::Database, state::AppState};

use super::update::RoleConnectionUpdater;

static COOKIE_NAME: &str = "oauth_state";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/linked-roles", get(redirect_to_oauth))
        .route("/discord-oauth-callback", get(callback))
}

async fn redirect_to_oauth(State(client): State<BasicClient>) -> impl IntoResponse {
    let (url, state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes([
            Scope::new("identify".into()),
            Scope::new("role_connections.write".into()),
        ])
        .add_extra_param("prompt", "consent")
        .url();

    let cookie = format!(
        "{}={}; HttpOnly; Path=/; SameSite=Lax; Secure",
        COOKIE_NAME,
        state.secret()
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    (headers, Redirect::to(url.as_str()))
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    code: String,
    state: String,
}

async fn callback(
    Query(query): Query<AuthRequest>,
    State(oauth_client): State<BasicClient>,
    State(pool): State<PgPool>,
    State(state): State<AppState>,
    jar: CookieJar,
) -> impl IntoResponse {
    let cookie_state = jar.get(COOKIE_NAME).unwrap().value();
    if query.state != cookie_state {
        return StatusCode::FORBIDDEN;
    }

    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .unwrap();

    let access_token = format!("Bearer {}", token.access_token().secret());
    let discord_client = TwilightClient::new(access_token);

    let current_authorization = discord_client
        .current_authorization()
        .await
        .unwrap()
        .model()
        .await
        .unwrap();

    // Safe to unwrap because we always request the `identify` scope
    let id = current_authorization.user.unwrap().id;

    // Safe to unwrap because we assume Discord returns all the necessary fields
    pool.write_token(id, token.try_into().unwrap())
        .await
        .unwrap(); // TODO: don't

    state.update_role_connection(id).await.unwrap();

    StatusCode::OK
}
