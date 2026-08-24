use std::env;
use tracing_subscriber::{
    Registry,
    filter::Targets,
    fmt,
    layer::{Layered, SubscriberExt},
    util::SubscriberInitExt,
};

fn create_registry() -> Layered<Targets, Registry> {
    let default_log = if cfg!(debug_assertions) {
        "warn,tetanes=debug,tetanes-core=debug"
    } else {
        "warn,tetanes=info,tetanes-core=info"
    };
    let default_filter = default_log.parse::<Targets>().unwrap_or_default();

    tracing_subscriber::registry().with(
        env::var("RUST_LOG")
            .ok()
            .and_then(|filter| filter.parse::<Targets>().ok())
            .unwrap_or(default_filter),
    )
}

/// Initialize logging.
pub fn init() {
    let registry = create_registry();
    let registry = registry.with(
        fmt::layer()
            .compact()
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_writer(std::io::stderr),
    );

    if let Err(err) = registry.try_init() {
        tracing::warn!(?err, "tracing init failed");
    }
}
