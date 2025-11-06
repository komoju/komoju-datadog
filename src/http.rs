//! HTTP-related utilities

use http::HeaderMap;
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;
use tracing_datadog::http::DistributedTracingContext;

/// Returns a Datadog-style path group from a request path, with dynamic segments replaced by '?'.
///
/// # Examples
///
/// ```
/// # use komoju_datadog::http::path_group;
///
/// assert_eq!(
///   path_group("/api/v1/merchants/abc123/settlements"),
///   "/api/v1/merchants/?/settlements"
/// );
/// ```
#[inline]
pub fn path_group(path: &str) -> String {
    path.split('/')
        .map(|segment| {
            if STATIC_SEGMENT_RE.is_match(segment) {
                segment
            } else {
                "?"
            }
        })
        .join("/")
}

/// Regular expression that matches static segments in request paths, e.g. "api" or "v1".
static STATIC_SEGMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(?:[^0-9]*|v[0-9]+)$").expect("invalid static segment regex"));

/// Attaches tracing headers to a request's [`HeaderMap`], so that the far side can continue the
/// current trace.
///
/// # Examples
///
/// ```
/// use komoju_datadog::http::attach_tracing_headers;
///
/// let mut request = http::Request::builder().body("").unwrap();
/// attach_tracing_headers(request.headers_mut());
/// ```
pub fn attach_tracing_headers(headers: &mut HeaderMap) {
    headers.extend(tracing::Span::current().get_context().to_w3c_headers());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_segment_re_works() {
        assert!(STATIC_SEGMENT_RE.is_match(""));
        assert!(STATIC_SEGMENT_RE.is_match("v1"));
        assert!(STATIC_SEGMENT_RE.is_match("api"));
        assert!(!STATIC_SEGMENT_RE.is_match("abc123"));
        assert!(!STATIC_SEGMENT_RE.is_match("v123abc"));
    }
}
