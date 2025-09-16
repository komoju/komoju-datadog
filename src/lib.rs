#![doc = include_str!("../README.md")]

mod aws;

pub mod axum;
pub mod config;
pub mod statsd;
pub mod tracing;

pub use config::Config;
pub use tracing::Tracer;

pub use sqlx_datadog::instrument_query;
