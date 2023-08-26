mod commands;
mod components;
mod context;
pub mod register;

use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ed25519_dalek::Verifier;
use hyper::body::to_bytes;
use thiserror::Error;
use tracing::{debug, error};
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};

use crate::{scratch::ScratchAPIError, state::AppState};

use self::components::CustomIdError;

#[derive(Debug, Error)]
pub enum InteractionHandlerError {
    #[error("invalid signature headers")]
    InvalidSignatureHeaders,
    #[error("invalid signature")]
    InvalidSignature,
    #[error(transparent)]
    Body(#[from] hyper::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Interaction(#[from] InteractionError),
}

impl IntoResponse for InteractionHandlerError {
    fn into_response(self) -> Response {
        error!("{}", self);

        match self {
            Self::InvalidSignatureHeaders => StatusCode::BAD_REQUEST,
            Self::InvalidSignature => StatusCode::UNAUTHORIZED,
            Self::Body(_) => StatusCode::BAD_REQUEST,
            Self::SerdeJson(_) => StatusCode::BAD_REQUEST,
            Self::Interaction(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

#[derive(Debug, Error)]
pub enum InteractionError {
    #[error("unsupported interaction type: {0:?}")]
    UnsupportedType(InteractionType),
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error(transparent)]
    CustomId(#[from] CustomIdError),
    #[error(transparent)]
    ScratchAPI(#[from] ScratchAPIError),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

pub async fn interaction_handler(
    State(state): State<AppState>,
    req: axum::http::Request<Body>,
) -> Result<Json<InteractionResponse>, InteractionHandlerError> {
    let headers = req.headers();
    let signature = headers
        .get("x-signature-ed25519")
        .and_then(|v| v.to_str().ok())
        .ok_or(InteractionHandlerError::InvalidSignatureHeaders)?
        .parse()
        .map_err(|_| InteractionHandlerError::InvalidSignatureHeaders)?;

    let timestamp = req
        .headers()
        .get("x-signature-timestamp")
        .ok_or(InteractionHandlerError::InvalidSignatureHeaders)?
        .to_owned();

    let body_bytes = to_bytes(req).await?;

    state
        .config
        .public_key
        .verify(
            vec![timestamp.as_bytes(), &body_bytes].concat().as_ref(),
            &signature,
        )
        .map_err(|_| InteractionHandlerError::InvalidSignature)?;

    let interaction = serde_json::from_slice::<Interaction>(&body_bytes)?;

    let res = router(interaction, state).await?;

    Ok(Json(res))
}

async fn router(
    interaction: Interaction,
    state: AppState,
) -> Result<InteractionResponse, InteractionError> {
    debug!("{:?}", interaction);
    let locale = interaction.locale.clone().into();

    match interaction.kind {
        InteractionType::Ping => Ok(InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None,
        }),
        InteractionType::ApplicationCommand => {
            commands::router(state, interaction.into(), locale).await
        }
        InteractionType::MessageComponent => {
            components::router(state, interaction.into(), locale).await
        }
        kind => Err(InteractionError::UnsupportedType(kind)),
    }
}
