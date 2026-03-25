## [0.6.1] - 2026-03-25

### 💼 Other

- *(deps)* Bump tracing from 0.1.43 to 0.1.44 (#28)
- *(deps)* Bump serde_json from 1.0.145 to 1.0.146 (#29)
- *(deps)* Bump axum from 0.8.7 to 0.8.8 (#30)
- *(deps)* Bump serde_json from 1.0.146 to 1.0.148 (#33)
- *(deps)* Bump reqwest from 0.12.26 to 0.13.1 (#34)
- *(deps)* Bump tokio from 1.48.0 to 1.49.0 (#35)
- *(deps)* Bump tracing-datadog from 0.6.0 to 0.6.1 (#36)
- *(deps)* Bump url from 2.5.7 to 2.5.8 (#37)
- *(deps)* Bump serde_json from 1.0.148 to 1.0.149 (#38)
- *(deps)* Bump tower from 0.5.2 to 0.5.3 (#39)
- *(deps)* Bump regex from 1.12.2 to 1.12.3 (#40)
- *(deps)* Bump reqwest from 0.13.1 to 0.13.2 (#41)
- *(deps)* Bump dogstatsd from 0.12.1 to 0.12.2 (#42)
- *(deps)* Bump pin-project-lite from 0.2.16 to 0.2.17 (#45)
- *(deps)* Bump futures-util from 0.3.31 to 0.3.32 (#43)
- *(deps)* Bump dogstatsd from 0.12.2 to 0.12.3 (#44)
- *(deps)* Bump tokio from 1.49.0 to 1.50.0 (#46)
- *(deps)* Bump tracing-subscriber from 0.3.22 to 0.3.23 (#47)
- Bump sqlx-datadog to 0.4.2

### 📚 Documentation

- Update tracing documentation
## [0.6.0] - 2025-12-18

### 🚀 Features

- Add support for 128-bit trace IDs

### 💼 Other

- *(deps)* Bump reqwest from 0.12.24 to 0.12.25 (#26)
- *(deps)* Bump reqwest from 0.12.25 to 0.12.26 (#27)
- Properly tag the license as MIT

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.6.0
## [0.5.1] - 2025-12-04

### 🐛 Bug Fixes

- Add timeout to blocking reqwest call to AWS SDK (#25)

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.5.1
## [0.5.0] - 2025-12-01

### 🐛 Bug Fixes

- [**breaking**] Perform some level of URL validation
- Refuse empty service names in config

### 💼 Other

- *(deps)* Bump the otel group with 2 updates (#24)

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.5.0
## [0.4.11] - 2025-11-26

### 🚀 Features

- Add container correlation support for GCP/GKE

### 💼 Other

- *(deps)* Bump tracing-datadog from 0.5.0 to 0.5.1 in the otel group
- *(deps)* Bump http from 1.3.1 to 1.4.0
- *(deps)* Bump axum from 0.8.6 to 0.8.7

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.11
## [0.4.10] - 2025-11-24

### 💼 Other

- *(deps)* Upgrade tracing-datadog to 0.5

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.10
## [0.4.9] - 2025-11-16

### 💼 Other

- *(deps)* Update tracing-datadog to 0.4

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.9
## [0.4.8] - 2025-11-14

### 🐛 Bug Fixes

- *(axum)* Set span.kind to server for axum handler spans

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.8
## [0.4.7] - 2025-11-14

### 🐛 Bug Fixes

- *(axum)* Don't overwrite the operation with an empty route

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.7
## [0.4.6] - 2025-11-14

### 💼 Other

- *(deps)* Update tracing-datadog to 0.3.3

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.6
## [0.4.5] - 2025-11-14

### 💼 Other

- *(deps)* Update tracing-datadog to 0.3.2

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.5
## [0.4.4] - 2025-11-13

### 💼 Other

- *(deps)* Update tracing-datadog to 0.3.0

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.4
## [0.4.3] - 2025-11-13

### 🐛 Bug Fixes

- Enable structured logs outside of development environments

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.3
## [0.4.2] - 2025-11-07

### 📚 Documentation

- Remove the outdated installation section from the readme
- Correctly downcase the second D in Datadog

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.2
## [0.4.1] - 2025-11-07

### 🐛 Bug Fixes

- [**breaking**] Correctly set and document the trace agent URL

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.1
## [0.4.0] - 2025-11-06

### 🚀 Features

- [**breaking**] Move from tracing-opentelemetry to tracing-datadog

### 💼 Other

- *(deps)* Bump reqwest from 0.12.23 to 0.12.24
- *(deps)* Bump regex from 1.12.1 to 1.12.2
- *(deps)* Bump tokio from 1.47.1 to 1.48.0

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.4.0
## [0.3.0] - 2025-10-14

### 🚀 Features

- *(axum)* Add DataDog identity tracking tags to axum spans

### 💼 Other

- *(deps)* Bump regex from 1.11.3 to 1.12.1

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.3.0
## [0.2.3] - 2025-10-08

### 🐛 Bug Fixes

- Don't disable tracing altogether without DD_AGENT_URL

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.2.3
## [0.2.2] - 2025-10-06

### 📚 Documentation

- Fix the documented default value for Config's trace_agent_url

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.2.2
## [0.2.1] - 2025-10-03

### 💼 Other

- *(deps)* Bump axum from 0.8.4 to 0.8.6
- *(deps)* Bump regex from 1.11.2 to 1.11.3

### ⚙️ Miscellaneous Tasks

- Release komoju-datadog version 0.2.1
## [0.2.0] - 2025-10-02

### 🚀 Features

- [**breaking**] Make the trace agent URL optional and default to none

### 🐛 Bug Fixes

- Leave the default resource name blank

### 🚜 Refactor

- Use a hand-written builder instead of Bon

### 📚 Documentation

- Flesh out the documentation with more examples
- Link to docs.rs in the readme

### ⚙️ Miscellaneous Tasks

- Create GitHub releases on every tag push
- Install rustfmt & clippy in CI
- Release komoju-datadog version 0.2.0
## [0.1.1] - 2025-09-16

### 🚀 Features

- Port over the initial implementation from Omega Star
- Make support for AWS, axum, and SQLx optional
- Add a function to propagate tracing headers
- Improve ergonomics of the statsD client

### 🐛 Bug Fixes

- Show human-readable logs in development env

### 💼 Other

- Add required attributes to Cargo.toml to publish

### 📚 Documentation

- Better doctests
- Flesh out the README a bit
- Add a changelog

### ⚙️ Miscellaneous Tasks

- Add a release.toml for cargo-release
- Set up a CI workflow
- Add a license
- Release komoju-datadog version 0.1.1
