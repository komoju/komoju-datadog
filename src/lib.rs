#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "aws_ecs")]
mod aws;

#[cfg(feature = "gcp_gke")]
mod gcp;

pub mod config;
pub mod http;
pub mod statsd;
pub mod tracing;

#[cfg(feature = "axum")]
pub mod axum;

pub use config::Config;

#[cfg(feature = "sqlx")]
pub use sqlx_datadog::instrument_query;
