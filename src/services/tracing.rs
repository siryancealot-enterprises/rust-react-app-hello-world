//! Provides utilities to initialize tracing and logging and provide functions to interact with it.

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer};

/// Initialize tracing services (which supports logging and other related things)
pub fn init_tracing() {
    let stdout_log = fmt::layer();

    tracing_subscriber::registry()
        .with(Layer::with_filter(stdout_log, LevelFilter::TRACE))
        // We can andd more layers here with different filter levels to send to log files, metrics service, etc.
        .init();
}
