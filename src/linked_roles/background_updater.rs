use std::time::Duration;

use time::OffsetDateTime;
use tokio::{
    task::JoinHandle,
    time::{interval, MissedTickBehavior},
};
use tracing::{debug, error, info};

use crate::{database::Database, state::AppState};

use super::RoleConnectionUpdater;

pub fn spawn(state: AppState) -> JoinHandle<()> {
    tokio::spawn(background_updater(state))
}

async fn background_updater(state: AppState) -> () {
    debug!("starting background updater");

    let start_time = OffsetDateTime::now_utc().time();
    let mut day = interval(Duration::from_secs(60 * 60 * 24));
    day.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        day.tick().await;

        info!("starting today's background metadata update");

        let today = OffsetDateTime::now_utc().replace_time(start_time);

        let mut delay = interval(Duration::from_secs(10));
        // Ensure at least 10 seconds for every batch of ScratchDB calls
        delay.set_missed_tick_behavior(MissedTickBehavior::Delay);

        let mut successful = 0;
        let mut failed = 0;

        // Update cached metadata records starting from the oldest
        // until all records have been updated today
        // Records added today will be alredy up to date on creation
        loop {
            match update_next_metadata(&state, today).await {
                Ok(Some(())) => successful += 1,
                Ok(None) => break,
                Err(err) => {
                    error!("{}", err);
                    failed += 1;
                }
            }

            delay.tick().await;
        }

        info!(
            "updated metadata ({successful} successful, {failed} failed), waiting until tomorrow",
        );
    }
}

async fn update_next_metadata(
    state: &AppState,
    today: OffsetDateTime,
) -> anyhow::Result<Option<()>> {
    if let Some((id, updated_at)) = state.pool.get_oldest_metadata().await? {
        if today > updated_at {
            debug!("updating role connection metadata for {id}");
            state.update_role_connection(id).await?;
            Ok(Some(()))
        } else {
            debug!("{id} was already updated today at {}", updated_at.time());
            Ok(None)
        }
    } else {
        debug!("no records to update");
        Ok(None)
    }
}
