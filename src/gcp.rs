//! GCP-related functionality

use std::sync::OnceLock;

/// The pod UID (unique identifier for the pod)
static POD_UID: OnceLock<Option<String>> = OnceLock::new();

/// Returns a unique identifier for this container instance.
///
/// For GKE/Kubernetes, we use the Pod UID which is available via Downward API.
/// This is a UUID that uniquely identifies the pod instance (e.g., c5b7b233-ca3d-4879-b55e-3d8e655fd044).
///
/// Note: The actual container ID (containerd://...) is NOT available via Downward API
/// due to Kubernetes limitations. The Pod UID is the next best option for correlation.
///
/// Falls back to pod name (HOSTNAME) if POD_UID is not set.
pub(crate) fn pod_uid() -> Option<&'static str> {
    POD_UID
        .get_or_init(|| {
            // Try to get Pod UID from environment variable (set via Downward API)
            std::env::var("POD_UID").ok().or_else(|| {
                // Fallback to pod name (always available via HOSTNAME)
                std::env::var("HOSTNAME").ok()
            })
        })
        .as_ref()
        .map(String::as_str)
}
