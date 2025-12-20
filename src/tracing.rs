//! Tracing from `tracing` via OpenTelemetry
//!
//! Like Rust's `tracing`, this handles both logs and trace spans.
//!
//! Logs are emitted to stdout, in a format that's dependent on the environment (human-readable
//! for `development`, JSON for anything else).
//!
//! Traces are emitted to the Datadog agent.

#[cfg(all(feature = "aws_ecs", feature = "gcp_gke"))]
compile_error!(
    "Features 'aws_ecs' and 'gcp_gke' are mutually exclusive and cannot be enabled together"
);

use tracing_datadog::DatadogTraceLayer;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Tracing instrumentation. Should be initialized exactly once to install the global handlers.
///
/// # Examples
///
/// ```
/// use komoju_datadog::{Config, tracing::Tracer};
///
/// #[tokio::main]
/// async fn main() {
///    let o11y_config = Config::builder().build().expect("invalid config");
///    Tracer::new(&o11y_config);
///
///    // ...
/// }
/// ```
#[allow(clippy::needless_doctest_main)]
pub struct Tracer;

impl Tracer {
    /// Initializes tracing instrumentation.
    pub fn new(config: &crate::Config) -> Self {
        let dd_trace_layer = match &config.trace_agent_url {
            Some(trace_agent_url) => {
                #[cfg_attr(not(any(feature = "aws_ecs", feature = "gcp_gke")), allow(unused_mut))]
                let mut builder = DatadogTraceLayer::builder()
                    .service(&config.service)
                    .env(&config.env)
                    .version(&config.version)
                    .agent_address(trace_agent_url)
                    .enable_logs(config.env != "development");
                #[cfg(feature = "aws_ecs")]
                if let Some(container_id) = crate::aws::container_id() {
                    builder = builder.container_id(container_id);
                }
                #[cfg(feature = "gcp_gke")]
                if let Some(pod_uid) = crate::gcp::pod_uid() {
                    builder = builder.container_id(pod_uid);
                }
                Some(
                    builder
                        .build()
                        .expect("failed to build Datadog trace layer"),
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
