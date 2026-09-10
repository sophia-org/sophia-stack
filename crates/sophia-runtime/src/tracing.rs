use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceLevel {
    Info,
    Debug,
    Trace,
}

impl TraceLevel {
    const fn filter(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TracingInitError {
    AlreadyInitialized,
}

impl SophiaErrorExt for TracingInitError {
    fn kind(&self) -> SophiaErrorKind {
        SophiaErrorKind::RuntimeAlreadyInitialized
    }
}

impl fmt::Display for TracingInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyInitialized => f.write_str("tracing subscriber is already initialized"),
        }
    }
}

impl std::error::Error for TracingInitError {}

pub fn init_tracing(level: TraceLevel) -> Result<(), TracingInitError> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.filter()));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .map_err(|_| TracingInitError::AlreadyInitialized)
}

/// Install a binary-owned observation layer independently of the console filter.
pub fn init_tracing_with_layer<L>(level: TraceLevel, layer: L) -> Result<(), TracingInitError>
where
    L: tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync + 'static,
{
    use tracing_subscriber::prelude::*;

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.filter()));

    tracing_subscriber::registry()
        .with(layer)
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .try_init()
        .map_err(|_| TracingInitError::AlreadyInitialized)
}
