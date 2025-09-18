//! Configuration

/// Observability configuration.
///
/// This is used to configure how data is sent to Datadog, as well as default tags.
///
/// Fields can be explicitly set through the builder or set via `DD_` environment variables.
///
/// # Examples
///
/// ```
/// use komoju_datadog::Config;
///
/// Config::builder()
///   .service("my-service")
///   .env("production")
///   .version("1.2.3")
///   .build();
/// ```
#[derive(Debug, bon::Builder)]
#[non_exhaustive]
pub struct Config {
    /// The `service` tag to use for metrics and traces.
    ///
    /// Can also be set via the `DD_SERVICE` environment variable.
    ///
    /// Defaults to `unknown`.
    #[builder(into, default = default_service())]
    pub service: String,

    /// The `env` tag to use for metrics and traces.
    ///
    /// Can also be set via the `DD_ENV` environment variable.
    ///
    /// Defaults to `development`.
    #[builder(into, default = default_env())]
    pub env: String,

    /// The `version` tag to use for metrics and traces.
    ///
    /// Can also be set via the `DD_VERSION` environment variable.
    ///
    /// Defaults to `unknown`.
    #[builder(into, default = default_version())]
    pub version: String,

    /// The Datadog agent URL to send traces to.
    ///
    /// Can also be set via the `DD_TRACE_AGENT_URL` environment variable.
    ///
    /// Defaults to `http://localhost:8126`.
    #[builder(into, default = default_trace_agent_uri())]
    pub trace_agent_url: String,

    /// The Datadog agent URL to send statsD metrics to.
    ///
    /// Can also be set via the `DD_METRICS_AGENT_URL` environment variable.
    ///
    /// Defaults to `localhost:8125`.
    #[builder(into, default = default_metrics_agent_uri())]
    pub metrics_agent_url: String,
}

fn default_service() -> String {
    std::env::var("DD_SERVICE").unwrap_or_else(|_| String::from("unknown"))
}

fn default_env() -> String {
    std::env::var("DD_ENV").unwrap_or_else(|_| String::from("development"))
}

fn default_version() -> String {
    std::env::var("DD_VERSION").unwrap_or_else(|_| String::from("unknown"))
}

fn default_trace_agent_uri() -> String {
    std::env::var("DD_TRACE_AGENT_URL").unwrap_or_else(|_| String::from("http://localhost:8126"))
}

fn default_metrics_agent_uri() -> String {
    std::env::var("DD_METRICS_AGENT_URL").unwrap_or_else(|_| String::from("localhost:8125"))
}
