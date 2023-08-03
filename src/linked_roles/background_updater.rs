use std::time::Duration;

use time::OffsetDateTime;
use tokio::{
    task::JoinHandle,
    time::{interval, MissedTickBehavior},
};
use tracing::{debug, error};

use crate::{database::Database, state::AppState};

use super::RoleConnectionUpdater;

pub fn spawn(state: AppState) -> JoinHandle<()> {
    tokio::spawn(background_updater(state))
}

async fn background_updater(state: AppState) -> () {
    debug!("starting background updater");

    let start_time = OffsetDateTime::now_utc().time();
    let mut day = interval(Duration::from_secs(60 * 60 * 24));
    let mut last_updated_at = OffsetDateTime::now_utc();

    loop {
        let today = OffsetDateTime::now_utc().replace_time(start_time);

        let mut delay = interval(Duration::from_secs(10));
        // Ensure at least 10 seconds for every batch of ScratchDB calls
        delay.set_missed_tick_behavior(MissedTickBehavior::Delay);

        // Update cached metadata records starting from the oldest
        // until all records have been updated today
        // Records added in the meantime will be alredy up to date on creation
        while today > last_updated_at {
            if let Err(err) = update_next_metadata(&state, &mut last_updated_at).await {
                error!("{}", err);
            }

            delay.tick().await;
        }

        debug!("updated all records, waiting until tomorrow");

        day.tick().await;
    }
}

async fn update_next_metadata(
    state: &AppState,
    last_updated_at: &mut OffsetDateTime,
) -> anyhow::Result<()> {
    if let Some((id, updated_at)) = state.pool.get_oldest_metadata().await? {
        debug!("updating role connection metadata for {}", id);
        state.update_role_connection(id).await?;
        *last_updated_at = updated_at;
    } else {
        debug!("no records to update");
        // Prevent infinite loop
        *last_updated_at = OffsetDateTime::now_utc();
    };
    Ok(())
}
