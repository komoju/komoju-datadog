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
