# KOMOJU Datadog

This is our in-house Datadog integration for Rust. It is similar to Datadog's
own integrations for other languages, providing support for the following:

- Logs and tracing via `tracing`, with automatic correlation
- StatsD metrics
- Axum integration, automatic tracing for each request
- Simple header injection for distributed tracing across HTTP requests
- AWS ECS container correlation, container metrics
- SQLx integration, correct tracing for SQL queries

## Usage

This library is configured through the `Config` struct, which automatically
picks up `DD_` environment variables. See
the [API documentation](https://docs.rs/komoju-datadog/latest/komoju_datadog/)
for a complete list of options.

```rust
use komoju_datadog::{Config, tracing::Tracer, statsd::StatsD};

fn main() {
    let o11y_config = Config::builder().service("my-service").build().expect("invalid config");
    StatsD::init_global(&o11y_config);
    Tracer::new(&o11y_config);

    // ...
}
```

### Tracing

Once the tracer is initialized, `tracing` spans can be used to create spans on
Datadog, either manually or via the `instrument` macro.

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

Certain span tags have special semantic meaning in Datadog:

- `operation` will be mapped to the operation name in Datadog, defaulting to
  the span name otherwise
- `resource` will be mapped to the resource name in Datadog

#### Errors

Errors can be rendered in Datadog by including the following tags:

- `error.type`
- `error.msg`
- `error.stack` (optional)

### Metrics

Metrics can be sent to Datadog using the `StatsD` struct. A global instance is
available via `StatsD::global()` after initialization.

```rust
use komoju_datadog::{Config, statsd::StatsD};

StatsD::init_global(&Config::builder().build().expect("invalid config"));

let _ = StatsD::global().incr("questions.answered", &["answer:42"]);
```

## Features

This crate has several optional features which can be enabled:

| feature   | use case                                                        | requirements |
|-----------|-----------------------------------------------------------------|--------------|
| `ahash`   | slightly better performance for one extra dependency            | - |
| `aws_ecs` | Running on AWS ECS/Fargate                                      | Requires `ECS_CONTAINER_METADATA_URI_V4` env var |
| `axum`    | Using Axum web framework                                        | - |
| `gcp_gke` | Running on GCP GKE                                              | Requires `POD_UID` env var via Downward API |
| `sqlx`    | Using SQLx for database queries                                 | - |

See the API documentation for more details and usage examples.

### GKE/Kubernetes Setup

To enable container correlation on GKE, configure the Downward API in your pod spec:

```yaml
env:
  - name: POD_UID
    valueFrom:
      fieldRef:
        fieldPath: metadata.uid
```
