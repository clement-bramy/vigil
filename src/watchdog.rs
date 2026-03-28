use std::time::Duration;

use sd_notify::NotifyState;
use tokio::time::sleep;
use tracing::{error, info, trace};

pub async fn watchdog(interval: Duration) {
    if let Err(err) = sd_notify::notify(&[NotifyState::Ready]) {
        error!("Failed to notify readiness: {}", err);
        return;
    }

    info!("Emitting watchdog even every: {:?}", interval);
    loop {
        if let Err(err) = sd_notify::notify(&[NotifyState::Watchdog]) {
            error!("Failed to update watchdog: {}", err);
            break;
        }

        trace!("WATCHDOG=1");
        sleep(interval).await;
    }
}
