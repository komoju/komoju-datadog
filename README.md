# KOMOJU DataDog

This is our in-house DataDog integration for Rust. It is similar to DataDog's
own integrations for other languages, providing support for the following:

- Logs and tracing via `tracing`, with automatic correlation
- StatsD metrics
- Axum integration, automatic tracing for each request
- Simple header injection for distributed tracing across HTTP requests
- AWS ECS container correlation, container metrics
- SQLx integration, correct tracing for SQL queries

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
komoju-datadog = "0.1"
```

## Usage

This library is configured through the `Config` struct, which automatically
picks up `DD_` environment variables. See
the [API documentation](https://docs.rs/komoju-datadog/latest/komoju_datadog/)
for a complete list of options.

```rust
use komoju_datadog::{Config, tracing::Tracer, statsd::StatsD};

fn main() {
    let o11y_config = Config::builder().service("my-service").build();
    StatsD::init_global(&o11y_config);
    Tracer::new(&o11y_config);

    // Start the service here, e.g. via tokio::runtime::Runtime::block_on.
}
```

### Tracing

Once the tracer is initialized, `tracing` spans can be used to create spans on
DataDog, either manually or via the `instrument` macro.

```rust
use tracing::{info, info_span, instrument};

#[instrument(err)]
fn answer(question: &str) -> Result<u64, &'static str> {
    info!(question, "Answering the question");

    let answer = {
        // This span lives for the duration of this block.
        let _span = info_span!("deep_thought").entered();
        std::thread::sleep(std::time::Duration::from_secs(u64::MAX));
        42
    };

    if answer != 42 {
        return Err("Got the wrong answer");
    }

    Ok(answer)
}
```

Certain span tags have special semantic meaning in DataDog:

- `operation` will be mapped to the operation name in DataDog, defaulting to 
  the span name otherwise
- `resource` will be mapped to the resource name in DataDog

#### Errors

Errors can be rendered in DataDog by including the following tags:

- `error.type`
- `error.msg`
- `error.stack` (optional)

### Metrics

Metrics can be sent to DataDog using the `StatsD` struct. A global instance is
available via `StatsD::global()` after initialization.

```rust
use komoju_datadog::{Config, statsd::StatsD};

StatsD::init_global(&Config::builder().build());

let _ = StatsD::global().incr("questions.answered", &["answer:42"]);
```

## Features

This crate has several optional features which can be enabled:

| feature   | provides                                                        |
|-----------|-----------------------------------------------------------------|
| `aws_ecs` | correlation of traces to ECS containers, enables system metrics |
| `axum`    | `AxumTraceLayer` for automatic spans for each request           |
| `sqlx`    | `instrument_query` macro for tracing SQLx queries               |

See the API documentation for more details and usage examples.
