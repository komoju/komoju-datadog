//! `OpenTelemetry` http_server helper functions.
//!
//! Adapted from <https://github.com/will-bank/datadog-tracing>.

use axum::extract::{ConnectInfo, MatchedPath};
use http::{Request, Response};
use itertools::Itertools;
use pin_project_lite::pin_project;
use regex::Regex;
use std::{
    error::Error,
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::LazyLock,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{field::Empty, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_opentelemetry_instrumentation_sdk::{
    http::{self as otel_http, http_flavor, http_host, http_method, url_scheme, user_agent},
    TRACING_TARGET,
};

/// Creates a span from a request.
fn make_span_from_request<B>(req: &Request<B>) -> Span {
    let http_method = http_method(req.method());
    let client_ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok().map(|s| s.to_string()))
        .or_else(|| {
            req.extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|addr| addr.ip().to_string())
        });
    let request_id = req
        .headers()
        .get("X-Request-Id")
        .and_then(|h| h.to_str().ok().map(|s| s.to_string()));
    tracing::info_span!(
        target: TRACING_TARGET,
        "HTTP request",
        operation = "axum.request",
        resource = format!("{} {}", http_method, route_from_path(req.uri().path())),
        http.base_url = http_host(req),
        http.method = %http_method,
        http.url = req.uri().path(),
        http.useragent = user_agent(req),
        http.route = Empty,
        http.client.ip = client_ip,
        http.request_id = request_id,
        http.status_code = Empty,
        network.protocol.version = %http_flavor(req.version()),
        server.address = http_host(req),
        url.scheme = url_scheme(req.uri()),
        otel.name = %http_method,
        otel.kind = ?opentelemetry::trace::SpanKind::Server,
        otel.status_code = Empty,
        request_id = Empty,
        error.type = Empty,
        error.message = Empty,
        "span.type" = "web",
        span.kind = "server",
        auth.method = Empty,
        auth.user_uuid = Empty,
        auth.merchant_uuid = Empty,
        auth.account_uuid = Empty,
        auth.role = Empty,
        auth.api_version = Empty,
    )
}

/// Updates a span with tags from the response.
fn update_span_from_response<B>(span: &Span, response: &Response<B>) {
    span.record("http.status_code", response.status().as_u16());
}

/// Updates a span with tags from an error response.
fn update_span_from_error<E>(span: &Span, error: &E)
where
    E: Error,
{
    span.record("otel.status_code", "ERROR");
    span.record("error.type", error.to_string());
    error
        .source()
        .map(|s| span.record("error.message", s.to_string()));
}

/// Updates a span with tags from a response or error.
fn update_span_from_response_or_error<B, E>(span: &Span, response: &Result<Response<B>, E>)
where
    E: Error,
{
    match response {
        Ok(response) => update_span_from_response(span, response),
        Err(err) => update_span_from_error(span, err),
    }
}

/// Axum Layer to create OTel spans for requests.
///
/// # Example
///
/// ```
/// # use axum::routing::{Router, get, post};
/// use komoju_datadog::axum::OtelAxumLayer;
///
/// # let router: Router<()> =
/// axum::Router::new()
///   // Example route that creates a span for each request.
///   .route("/sign_in", post(sign_in))
///   .layer(OtelAxumLayer)
///   // No traces on health checks.
///   .route("/health_check", get(health_check));
///
/// # async fn sign_in() {}
/// # async fn health_check() {}
/// ```
#[derive(Clone, Debug)]
pub struct OtelAxumLayer;

impl<S> Layer<S> for OtelAxumLayer {
    /// The wrapped service
    type Service = OtelAxumService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        OtelAxumService { inner }
    }
}

/// Middleware `Service` layer that creates OTel spans for every request.
#[derive(Debug, Clone)]
pub struct OtelAxumService<S> {
    /// The inner service layer.
    inner: S,
}

impl<S, B, B2> Service<Request<B>> for OtelAxumService<S>
where
    S: Service<Request<B>, Response = Response<B2>> + Clone + Send + 'static,
    S::Error: Error + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let req = req;
        let span = {
            let span = make_span_from_request(&req);

            let route = http_route(&req);
            let method = http_method(req.method());

            span.record("http.route", route);
            span.record("otel.name", format!("{method} {route}").trim());
            span.set_parent(otel_http::extract_context(req.headers()));

            span
        };
        let future = {
            let _ = span.enter();
            self.inner.call(req)
        };
        ResponseFuture {
            inner: future,
            span,
        }
    }
}

pin_project! {
    pub struct ResponseFuture<F> {
        #[pin]
        inner: F,
        span: Span,
    }
}

impl<Fut, ResBody, E> Future for ResponseFuture<Fut>
where
    Fut: Future<Output = Result<Response<ResBody>, E>>,
    E: Error + 'static,
{
    type Output = Result<Response<ResBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _guard = this.span.enter();
        let result = futures_util::ready!(this.inner.poll(cx));
        update_span_from_response_or_error(this.span, &result);

        Poll::Ready(result)
    }
}

/// Returns the route that matched a request, or an empty string.
#[inline]
fn http_route<B>(req: &Request<B>) -> &str {
    req.extensions()
        .get::<MatchedPath>()
        .map_or_else(|| "", |mp| mp.as_str())
}

/// Regular expression that matches static segments in request paths, e.g. "api" or "v1".
static STATIC_SEGMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(?:[^0-9]*|v[0-9]+)$").expect("invalid static segment regex"));

/// Returns a Datadog-style route from a request path, with dynamic segments replaced by '?'.
///
/// Example:
/// `/api/v1/merchants/abc123/settlements` becomes `/api/v1/merchants/?/settlements`
#[inline]
pub fn route_from_path(path: &str) -> String {
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
