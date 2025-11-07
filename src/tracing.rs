//! Tracing from `tracing` via OpenTelemetry
//!
//! Like Rust's `tracing`, this handles both logs and trace spans.
//!
//! Logs are emitted to stdout, in a format that's dependent on the environment (human-readable
//! for `development`, JSON for anything else).
//!
//! Traces are emitted to the Datadog agent.

use tracing_datadog::DataDogTraceLayer;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// OpenTelemetry instrumentation. Should be initialized once to install the global handlers and
/// will de-initialize upon shutdown.
///
/// It is important to initialize this before initializing an async runtime. Failing to do so will
/// result in panicking.
///
/// # Examples
///
/// ```
/// use komoju_datadog::{Config, tracing::Tracer};
///
/// fn main() {
///    let o11y_config = Config::builder().build();
///    let _tracer = Tracer::new(&o11y_config);
///
///    tokio::runtime::Builder::new_multi_thread()
///       .max_blocking_threads(1024)
///       .enable_all()
///       .build()
///       .expect("failed to build runtime")
///       .block_on(server())
/// }
///
/// async fn server() {
///   // Start a server, do stuff.
/// }
/// ```
#[allow(clippy::needless_doctest_main)]
pub struct Tracer;

impl Tracer {
    /// Initializes tracing instrumentation.
    ///
    /// # Panics
    ///
    /// Panics if called from an async context.
    pub fn new(config: &crate::Config) -> Self {
        let dd_trace_layer = match &config.trace_agent_url {
            Some(trace_agent_url) => {
                #[cfg_attr(not(feature = "aws_ecs"), allow(unused_mut))]
                let mut builder = DataDogTraceLayer::builder()
                    .service(&config.service)
                    .env(&config.env)
                    .version(&config.version)
                    .agent_address(trace_agent_url);
                #[cfg(feature = "aws_ecs")]
                if let Some(container_id) = crate::aws::container_id() {
                    builder = builder.container_id(container_id);
                }
                Some(
                    builder
                        .build()
                        .expect("failed to build DataDog trace layer"),
                )
            }
            _ => None,
        };

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .with(if config.env == "development" {
                Some(tracing_subscriber::fmt::layer().pretty())
            } else {
                None
            })
            .with(dd_trace_layer)
            .init();

        Self
    }
}
