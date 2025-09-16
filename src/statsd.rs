//! StatsD metrics
//!
//! Metrics should use a single, shared, global client to submit metrics.

use std::sync::OnceLock;

/// Global DogstatsD client for submitting metrics.
pub static METRICS: OnceLock<dogstatsd::Client> = OnceLock::new();

/// Initializes the global DogstatsD client.
pub fn init(config: &crate::Config) {
    let dogstatsd_client = dogstatsd::Client::new(
        dogstatsd::OptionsBuilder::new()
            .to_addr(config.metrics_agent_url.clone())
            .default_tag(format!("service:{}", config.service))
            .default_tag(format!("env:{}", config.env))
            .default_tag(format!("version:{}", config.version))
            .build(),
    )
    .expect("failed to create DogstatsD client");

    METRICS
        .set(dogstatsd_client)
        .expect("failed to set DogstatsD client");
}
