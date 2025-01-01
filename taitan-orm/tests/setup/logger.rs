use std::sync::OnceLock;

static logger_setup: OnceLock<bool> = OnceLock::new();
pub fn setup_logger() {
    logger_setup.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        true
    });
}
