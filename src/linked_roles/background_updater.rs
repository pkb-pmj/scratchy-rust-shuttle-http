use std::time::Duration;

use tokio::time::interval;

use crate::state::AppState;

pub async fn background_updater(state: AppState) -> ! {
    let mut day = interval(Duration::from_secs(60 * 60 * 24));

    loop {
        day.tick().await;

        let mut interval = interval(Duration::from_secs(10));

        // Get all cached metadata records sorted by updated_at
        // Update them one by one
        // Records added in the meantime will be alredy up to date on creation
    }
}
