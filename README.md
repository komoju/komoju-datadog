# KOMOJU DataDog

This is our in-house DataDog integration for Rust. It is similar to DataDog's
own integrations for other languages, providing support for metrics, traces,
and logs. All data is automatically tagged to correlate logs and traces.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
komoju-datadog = "0.1"
```

## Usage

This library is configured through the `Config` struct, which automatically
picks up `DD_` environment variables.

```rust
use komoju_datadog::{Config, tracing::Tracer, statsd::StatsD};

fn main() {
    let o11y_config = Config::builder().service("my-service").build();
    StatsD::init_global(&o11y_config);
    let _tracer_guard = Tracer::new(&o11y_config);

    // Start the service here, e.g. via tokio::runtime::Runtime::block_on.
}
```

## Features

This crate has several optional features which can be enabled:

| feature   | provides                                                        |
|-----------|-----------------------------------------------------------------|
| `aws_ecs` | correlation of traces to ECS containers, enables system metrics |
| `axum`    | `AxumTraceLayer` for automatic spans for each request           |
| `sqlx`    | `instrument_query` macro for tracing SQLx queries               |

See the documentation for more details and usage examples.
