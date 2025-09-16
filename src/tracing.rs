//! Tracing from `tracing` via OpenTelemetry
//!
//! Like Rust's `tracing`, this handles both logs and trace spans.
//!
//! Logs are emitted to stdout, in a format that's dependent on the environment (human-readable
//! for `development`, JSON for anything else).
//!
//! Traces are emitted to the Datadog agent.

use datadog_formatting_layer::DatadogFormattingLayer;
#[cfg(feature = "aws_ecs")]
use http::{HeaderMap, HeaderValue};
use opentelemetry::trace::TracerProvider;
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
pub struct Tracer {
    /// Graceful shutdown handler, called when dropped.
    shutdown: Box<dyn Fn()>,
}

impl Tracer {
    /// Initializes tracing instrumentation.
    ///
    /// The returned value acts as a guard for RAII destruction.
    ///
    /// # Panics
    ///
    /// Panics if called from an async context.
    #[must_use = "Dropping the guard will de-initialize OpenTelemetry instrumentation"]
    pub fn new(config: &crate::Config) -> Self {
        let provider = opentelemetry_datadog::new_pipeline()
            .with_service_name(&config.service)
            .with_env(&config.env)
            .with_version(&config.version)
            .with_agent_endpoint(&config.trace_agent_url)
            .with_name_mapping(|span, _| {
                span.attributes
                    .iter()
                    .find(|k| k.key.as_str() == "operation")
                    .and_then(|kv| match &kv.value {
                        opentelemetry::Value::String(v) => Some(v.as_str()),
                        _ => None,
                    })
                    .unwrap_or(&*span.name)
            })
            .with_resource_mapping(|span, _| {
                span.attributes
                    .iter()
                    .find(|k| k.key.as_str() == "resource")
                    .and_then(|kv| match &kv.value {
                        opentelemetry::Value::String(v) => Some(v.as_str()),
                        _ => None,
                    })
                    .unwrap_or(&*span.name)
            })
            .with_http_client(
                #[cfg(feature = "aws_ecs")]
                reqwest::blocking::Client::builder()
                    .default_headers({
                        let mut headers = HeaderMap::new();
                        if let Some(container_id) = crate::aws::container_id() {
                            headers.insert(
                                "Datadog-Container-ID",
                                HeaderValue::from_static(container_id),
                            );
                        }
                        headers
                    })
                    .build()
                    .expect("failed to build OTel export reqwest client"),
                #[cfg(not(feature = "aws_ecs"))]
                reqwest::blocking::Client::new(),
            )
            .install_batch()
            .expect("failed to setup OpenTelemetry");

        opentelemetry::global::set_tracer_provider(provider.clone());
        opentelemetry::global::set_text_map_propagator(
            opentelemetry::propagation::composite::TextMapCompositePropagator::new(vec![
                Box::new(opentelemetry_datadog::DatadogPropagator::default()),
                Box::new(opentelemetry_sdk::propagation::TraceContextPropagator::default()),
            ]),
        );

        if config.version == "development" {
            tracing_subscriber::registry()
                .with(
                    tracing_subscriber::EnvFilter::builder()
                        .with_default_directive(LevelFilter::INFO.into())
                        .from_env_lossy(),
                )
                .with(tracing_subscriber::fmt::layer().pretty())
                .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("tracing")))
                .init();
        } else {
            tracing_subscriber::registry()
                .with(
                    tracing_subscriber::EnvFilter::builder()
                        .with_default_directive(LevelFilter::INFO.into())
                        .from_env_lossy(),
                )
                .with(DatadogFormattingLayer::default())
                .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("tracing")))
                .init();
        };

        Self {
            shutdown: Box::new(move || {
                provider
                    .shutdown()
                    .expect("failed to shutdown tracing provider");
            }),
        }
    }
}

impl Drop for Tracer {
    fn drop(&mut self) {
        (self.shutdown)();
    }
}
