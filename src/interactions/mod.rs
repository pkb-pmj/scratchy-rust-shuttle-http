mod commands;

use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ed25519_dalek::Verifier;
use hyper::body::to_bytes;
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};

use crate::state::AppState;

pub enum InteractionError {
    InvalidRequest,
    InvalidSignature,
    NotImplemented,
}

impl IntoResponse for InteractionError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidRequest => StatusCode::BAD_REQUEST,
            Self::InvalidSignature => StatusCode::UNAUTHORIZED,
            Self::NotImplemented => StatusCode::NOT_IMPLEMENTED,
        }
        .into_response()
    }
}

pub async fn interaction_handler(
    State(state): State<AppState>,
    req: axum::http::Request<Body>,
) -> Result<Json<InteractionResponse>, InteractionError> {
    let headers = req.headers();
    let signature = headers
        .get("x-signature-ed25519")
        .and_then(|v| v.to_str().ok())
        .ok_or(InteractionError::InvalidRequest)?
        .parse()
        .map_err(|_| InteractionError::InvalidRequest)?;

    let timestamp = req
        .headers()
        .get("x-signature-timestamp")
        .ok_or(InteractionError::InvalidRequest)?
        .to_owned();

    let body_bytes = to_bytes(req).await.unwrap();

    state
        .discord_public_key
        .verify(
            vec![timestamp.as_bytes(), &body_bytes].concat().as_ref(),
            &signature,
        )
        .map_err(|_| InteractionError::InvalidSignature)?;

    let interaction = serde_json::from_slice::<Interaction>(&body_bytes).unwrap();

    let res = router(interaction).await?;

    Ok(Json(res))
}

async fn router(interaction: Interaction) -> Result<InteractionResponse, InteractionError> {
    match interaction.kind {
        InteractionType::Ping => Ok(InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None,
        }),
        InteractionType::ApplicationCommand => match interaction.data {
            Some(InteractionData::ApplicationCommand(data)) => match data.name.as_str() {
                "ping" => commands::ping::run().await,
                _ => Err(InteractionError::NotImplemented),
            },
            _ => Err(InteractionError::InvalidRequest),
        },
        InteractionType::MessageComponent => match interaction.data {
            Some(InteractionData::MessageComponent(data)) => match data.custom_id.as_str() {
                _ => Err(InteractionError::NotImplemented),
            },
            _ => Err(InteractionError::InvalidRequest),
        },
        _ => Err(InteractionError::NotImplemented),
    }
}
