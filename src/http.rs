//! HTTP-related utilities

use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

/// Returns a Datadog-style path group from a request path, with dynamic segments replaced by '?'.
///
/// Example:
/// `/api/v1/merchants/abc123/settlements` becomes `/api/v1/merchants/?/settlements`
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
