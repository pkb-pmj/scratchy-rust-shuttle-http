use thiserror::Error;
use twilight_http::{response::DeserializeBodyError, Client, Error as TwilightHttpError};

use super::commands::{about, link, ping, user};

#[derive(Error, Debug)]
pub enum RegisterCommandsError {
    #[error(transparent)]
    DeserializeBody(#[from] DeserializeBodyError),
    #[error(transparent)]
    TwilightHttp(#[from] TwilightHttpError),
}

pub async fn register_commands(token: String) -> Result<(), RegisterCommandsError> {
    let client = Client::new(token);
    let application = client.current_user_application().await?.model().await?;
    let interaction_client = client.interaction(application.id);

    interaction_client
        .set_global_commands(&[
            about::register(),
            link::register(),
            ping::register(),
            user::register(),
        ])
        .await?
        .model()
        .await?;

    Ok(())
}
