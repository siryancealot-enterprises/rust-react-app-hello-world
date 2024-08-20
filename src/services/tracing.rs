use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize tracing services (which supports logging and other related things)
pub fn init_tracing() {
    let stdout_log = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_subscriber::Layer::with_filter(
            stdout_log,
            tracing_subscriber::filter::LevelFilter::TRACE,
        ))
        // We can andd more layers here with different filter levels to send to log files, metrics service, etc.
        .init();
}
