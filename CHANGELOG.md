## [0.4.5] - 2025-11-14

### 💼 Other

- *(deps)* Update tracing-datadog to 0.3.2
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
