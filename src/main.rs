use std::str::FromStr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

fn main() {
    let filter = tracing_subscriber::filter::EnvFilter::from_str("warn,collab=info").unwrap();

    tracing_subscriber::registry()
        .with(Box::new(
            tracing_subscriber::fmt::layer().with_filter(filter),
        ))
        .init();

    let _ = log::log_enabled!(
        target: "sqlx",
        log::Level::Info
    );

    tracing::error!("ERROR");
    tracing::warn!("WARN");
    tracing::info!("INFO");
}
