//! Configuration

use std::env;

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
#[derive(Debug)]
#[non_exhaustive]
pub struct Config {
    /// The `service` tag to use for metrics and traces.
    ///
    /// Can also be set via the `DD_SERVICE` environment variable.
    ///
    /// Defaults to `unknown`.
    pub service: String,

    /// The `env` tag to use for metrics and traces.
    ///
    /// Can also be set via the `DD_ENV` environment variable.
    ///
    /// Defaults to `development`.
    pub env: String,

    /// The `version` tag to use for metrics and traces.
    ///
    /// Can also be set via the `DD_VERSION` environment variable.
    ///
    /// Defaults to `unknown`.
    pub version: String,

    /// The Datadog agent URL to send traces to.
    ///
    /// Can also be set via the `DD_TRACE_AGENT_URL` environment variable.
    ///
    /// Defaults to `http://localhost:8126`.
    pub trace_agent_url: Option<String>,

    /// The Datadog agent URL to send statsD metrics to.
    ///
    /// Can also be set via the `DD_METRICS_AGENT_URL` environment variable.
    ///
    /// Defaults to `localhost:8125`.
    pub metrics_agent_url: String,
}

impl Config {
    /// Creates a new builder to construct a `Config`.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

/// Builder to construct a [`Config`].
pub struct ConfigBuilder {
    service: String,
    env: String,
    version: String,
    trace_agent_url: Option<String>,
    metrics_agent_url: String,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            service: env::var("DD_SERVICE").unwrap_or_else(|_| String::from("unknown")),
            env: env::var("DD_ENV").unwrap_or_else(|_| String::from("development")),
            version: env::var("DD_VERSION").unwrap_or_else(|_| String::from("unknown")),
            trace_agent_url: env::var("DD_TRACE_AGENT_URL").ok(),
            metrics_agent_url: env::var("DD_METRICS_AGENT_URL")
                .unwrap_or_else(|_| String::from("localhost:8125")),
        }
    }
}

impl ConfigBuilder {
    /// Sets the `service` for the config.
    ///
    /// By default, this is the value of `DD_SERVICE`, or otherwise `"unknown"`.
    pub fn service(mut self, service: impl Into<String>) -> Self {
        self.service = service.into();
        self
    }

    /// Sets the `env` for the config.
    ///
    /// By default, this is the value of `DD_ENV`, or otherwise `"development"`.
    pub fn env(mut self, env: impl Into<String>) -> Self {
        self.env = env.into();
        self
    }

    /// Sets the `version` for the config.
    ///
    /// By default, this is the value of `DD_VERSION`, or otherwise `"unknown"`.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Sets the `trace_agent_url` for the config.
    ///
    /// By default, this is the value of `DD_TRACE_AGENT_URL`, or otherwise `None`.
    pub fn trace_agent_url(mut self, trace_agent_url: Option<impl Into<String>>) -> Self {
        self.trace_agent_url = trace_agent_url.map(Into::into);
        self
    }

    /// Sets the `metrics_agent_url` for the config.
    ///
    /// By default, this is the value of `DD_METRICS_AGENT_URL`, or otherwise `"localhost:8126"`.
    pub fn metrics_agent_url(mut self, metrics_agent_url: impl Into<String>) -> Self {
        self.metrics_agent_url = metrics_agent_url.into();
        self
    }

    /// Consumes the builder, returning the constructed `Config`.
    pub fn build(self) -> Config {
        let Self {
            service,
            env,
            version,
            trace_agent_url,
            metrics_agent_url,
        } = self;

        Config {
            service,
            env,
            version,
            trace_agent_url,
            metrics_agent_url,
        }
    }
}
