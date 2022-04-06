use std::path::PathBuf;

use dirs::cache_dir;
use iced::{Application, Settings};
use refuel::pages::Refuel;
use tracing::{info, Level};
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

fn main() -> iced::Result {
    let _guard = setup_log();
    Refuel::run(Settings {
        antialiasing: true,
        ..Default::default()
    })?;
    Ok(())
}

fn setup_log() -> tracing_appender::non_blocking::WorkerGuard {
    let filter = filter::Targets::new()
        .with_target("iced", Level::INFO)
        .with_target("refuel", Level::TRACE);
    let file_appender = tracing_appender::rolling::daily(log_dir(), "refuel.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .pretty()
                .with_ansi(false),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();
    info!(
        "{} {} has started",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    guard
}

fn log_dir() -> PathBuf {
    let mut cache_dir = cache_dir().expect("could not find cache dir");
    cache_dir.push("refuel");
    cache_dir
}
