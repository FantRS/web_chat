use tracing::Level;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, filter::filter_fn, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn init_logger<S>(level: S)
where
    S: AsRef<str>,
{
    let stdout_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_level(true)
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_writer(std::io::stdout)
        .with_filter(filter_fn(|meta| {
            matches!(*meta.level(), Level::TRACE | Level::DEBUG | Level::INFO)
        }));

    let stderr_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_writer(std::io::stderr)
        .with_filter(filter_fn(|meta| {
            matches!(*meta.level(), Level::WARN | Level::ERROR)
        }));

    let env_layer = EnvFilter::try_from_default_env().unwrap_or(level.as_ref().into());

    Registry::default()
        .with(stdout_layer)
        .with(stderr_layer)
        .with(env_layer)
        .init();
}
