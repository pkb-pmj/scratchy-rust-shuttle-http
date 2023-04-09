use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ed25519_dalek::{PublicKey, Verifier};
use hyper::body::to_bytes;
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};

pub enum InteractionError {
    InvalidRequest,
    InvalidSignature,
}

impl IntoResponse for InteractionError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidRequest => StatusCode::BAD_REQUEST,
            Self::InvalidSignature => StatusCode::UNAUTHORIZED,
        }
        .into_response()
    }
}

pub async fn handle_interaction(
    State(public_key): State<PublicKey>,
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

    public_key
        .verify(
            vec![timestamp.as_bytes(), &body_bytes].concat().as_ref(),
            &signature,
        )
        .map_err(|_| InteractionError::InvalidSignature)?;

    let interaction = serde_json::from_slice::<Interaction>(&body_bytes).unwrap();

    let res = match interaction.kind {
        InteractionType::Ping => InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None,
        },
        _ => InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None,
        },
    };

    Ok(Json(res))
}
