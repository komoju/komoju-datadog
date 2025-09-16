//! AWS-related functionality

use std::sync::OnceLock;

/// The ID of the container we're running inside, if any.
static CONTAINER_ID: OnceLock<Option<String>> = OnceLock::new();

/// Returns the ID of the container we're running inside, if any.
///
/// Attempts to fetch ECS container metadata from the ECS Fargate v4 metadata endpoint on first
/// access. Because this is just used for o11y, we bail out and return `None` if anything goes
/// wrong.
///
/// # Panics
///
/// Panics if the initial call is from an async context.
pub(crate) fn container_id() -> Option<&'static str> {
    CONTAINER_ID
        .get_or_init(|| {
            let ecs_metadata_uri = std::env::var("ECS_CONTAINER_METADATA_URI_V4").ok()?;

            let container_id = reqwest::blocking::get(&ecs_metadata_uri)
                .ok()?
                .json::<serde_json::Value>()
                .ok()?
                .get("DockerId")?
                .as_str()?
                .to_string();

            Some(container_id)
        })
        .as_ref()
        .map(String::as_str)
}
