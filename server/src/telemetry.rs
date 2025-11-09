use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger<S>(level: S)
where
    S: AsRef<str>,
{
    let settings_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(false)
        .json()
        .with_span_list(false);

    let env_layer = EnvFilter::try_from_default_env().unwrap_or(level.as_ref().into());

    Registry::default()
        .with(settings_layer)
        .with(env_layer)
        .init();
}
