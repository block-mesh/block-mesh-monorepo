use once_cell::sync::OnceCell;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static INSTANCE: OnceCell<bool> = OnceCell::new();

pub fn setup_tracing() {
    if INSTANCE.get().is_some() {
        return;
    }
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .init();
    INSTANCE.set(true).unwrap()
}
