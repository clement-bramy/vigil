use std::{collections::HashSet, path::PathBuf};

use color_eyre::eyre::Result;
use notify::{Event, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

pub async fn watch(root: PathBuf) -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<notify::Result<Event>>(50);
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.blocking_send(res);
    })?;
    watcher.watch(&root, RecursiveMode::Recursive)?;
    debug!("watching root direcyory: {}", root.display());

    while let Some(res) = rx.recv().await {
        match res {
            Err(err) => error!("watch error: {:?}", err),
            Ok(event) => {
                // only handle changes for rust files
                let uniques: HashSet<PathBuf> = event
                    .paths
                    .into_iter()
                    .filter(|s| s.extension().is_some_and(|ex| ex == "rs"))
                    .collect();

                uniques
                    .iter()
                    .map(|path| (event.kind, path.display()))
                    .for_each(|(kind, path)| {
                        if kind.is_create() {
                            info!("Created: {}", path);
                        } else if kind.is_modify() {
                            info!("Modified: {}", path);
                        } else if kind.is_remove() {
                            info!("Removed: {}", path);
                        }
                    });
            }
        }
    }

    Ok(())
}
