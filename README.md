# KOMOJU DataDog

This is our in-house DataDog integration for Rust. It is similar to DataDog's
own integrations for other languages, providing support for metrics, traces,
and logs.

## Features

This crate has several optional features which can be enabled:

| feature   | gets you                                             |
|-----------|------------------------------------------------------|
| `aws_ecs` | correlation traces to ECS containers, system metrics |
| `axum`    | axum middleware for automatic spans for each request |
| `sqlx`    | `instrument_query` macro for tracing SQLx queries    |
