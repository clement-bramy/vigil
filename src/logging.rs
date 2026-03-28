use std::env;

use tokio::{
    select,
    signal::unix::{self, SignalKind},
    task::JoinHandle,
};
use tracing::error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Layer, Registry, layer::SubscriberExt, reload};

pub fn init_tracing() -> JoinHandle<()> {
    let env_filter = load_environment_filter();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_filter(env_filter);

    let (reloadable_fmt_layer, handle) = reload::Layer::new(fmt_layer);

    let journald_layer = tracing_journald::layer()
        .expect("journald socket")
        .with_filter(LevelFilter::DEBUG);

    let subscriber = Registry::default()
        .with(journald_layer)
        .with(reloadable_fmt_layer);
    tracing::subscriber::set_global_default(subscriber).expect("tracing setup");

    tokio::spawn(async move {
        let mut signal_user_1 = unix::signal(SignalKind::user_defined1()).expect("USR1 handler");
        let mut signal_user_2 = unix::signal(SignalKind::user_defined2()).expect("USR2 handler");

        loop {
            select! {
                _ = signal_user_1.recv() => {
                    if let Err(err) = handle.modify(|layer| *layer.filter_mut() = EnvFilter::new("trace")){
                        error!("Failed to update log level to TRACE: {}", err);
                    }
                },
                _ = signal_user_2.recv() =>{
                    if let Err(err) = handle.modify(|layer| *layer.filter_mut() = EnvFilter::new("info")){
                        error!("Failed to update log level to INFO: {}", err);
                    }
                },
            }
        }
    })
}

fn load_environment_filter() -> EnvFilter {
    if env::var("JOURNAL_STREAM").is_ok() {
        EnvFilter::new("off")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    }
}
