use std::{env, path::PathBuf, time::Duration};

use color_eyre::eyre::{Context, Result};
use tokio::select;
use tracing::error;

use crate::logging::init_tracing;

mod logging;
mod watchdog;
mod watcher;

#[tokio::main]
async fn main() {
    color_eyre::install().expect("panic hook setup");
    let signal_handle = init_tracing();

    if let Err(error) = run().await {
        error!("Vigil process failed: {}", error);
    }

    signal_handle.abort();
}

async fn run() -> Result<()> {
    let root = env::var("VIGIL_MONITOR_ROOT").wrap_err("Missing VIGIL_MONITOR_ROOT")?;
    let interval = env::var("WATCHDOG_USEC").wrap_err("Missing WATCHDOG_USEC")?;

    let root = PathBuf::from(root);
    let interval = interval
        .parse::<u64>()
        .map(|micros| Duration::from_micros(micros) / 2)
        .wrap_err("Invalid watchdog interval")?;

    select! {
        _ = watcher::watch(root) => {},
        _ =  watchdog::watchdog(interval) => {},
    }

    Ok(())
}
