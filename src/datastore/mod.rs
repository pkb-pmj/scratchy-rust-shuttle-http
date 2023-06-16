use sqlx::PgPool;
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Debug, Clone)]
pub struct ScratchUser {
    pub username: String,
    pub id: Id<UserMarker>,
}
