//! StatsD metrics
//!
//! This mostly proxies to the [`dogstatsd`] crate, which has more documentation on the API.

use std::{ops::Deref, sync::OnceLock};

/// Global StatsD instance, if used.
static GLOBAL_STATSD: OnceLock<StatsD> = OnceLock::new();

/// A client for submitting metrics to the Datadog agent.
///
/// Includes default tags for unified service tagging.
///
/// A single, global client is recommended, but a client can also be created and passed around
/// where needed.
///
/// # Examples
///
/// ```
/// use komoju_datadog::statsd::StatsD;
///
/// let config = komoju_datadog::Config::builder().build().expect("invalid config");
/// let statsd = StatsD::init_global(&config);
///
/// // From anywhere in the service.
/// let _ = StatsD::global().incr("my_counter", &["tag:counter"]);
/// ```
pub struct StatsD {
    inner: dogstatsd::Client,
}

impl StatsD {
    /// Creates a new StatsD client.
    ///
    /// Generally, one should use only one client for the entire service. Either pass it into
    /// the right places or use a static [`OnceLock`] to make it accessible from anywhere.
    /// See [`StatsD::init_global`] for a shortcut version.
    ///
    /// # Panics
    ///
    /// Can panic if bad options are passed.
    pub fn new(config: &crate::Config) -> Self {
        let inner = dogstatsd::Client::new(
            dogstatsd::OptionsBuilder::new()
                .to_addr(config.metrics_agent_url.clone())
                .default_tag(format!("service:{}", config.service))
                .default_tag(format!("env:{}", config.env))
                .default_tag(format!("version:{}", config.version))
                .build(),
        )
        .expect("failed to create DogstatsD client");

        Self { inner }
    }

    /// Creates a new global StatsD client.
    ///
    /// # Panics
    ///
    /// Can panic if bad options are passed.
    pub fn init_global(config: &crate::Config) -> &'static Self {
        GLOBAL_STATSD.get_or_init(|| StatsD::new(config))
    }

    /// Gets the global StatsD client.
    ///
    /// # Panics
    ///
    /// Panics if [`StatsD::init_global`] has not been called. Use [`StatsD::try_global`] for a
    /// non-panicking version.
    pub fn global() -> &'static Self {
        Self::try_global().expect("global StatsD client not initialized")
    }

    /// Attempts to get the global StatsD client.
    pub fn try_global() -> Option<&'static Self> {
        GLOBAL_STATSD.get()
    }
}

impl Deref for StatsD {
    type Target = dogstatsd::Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
