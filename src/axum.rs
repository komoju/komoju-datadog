//! `OpenTelemetry` http_server helper functions.
//!
//! Adapted from <https://github.com/will-bank/datadog-tracing>.

use axum::extract::{ConnectInfo, MatchedPath};
use http::{Request, Response, header};
use pin_project_lite::pin_project;
use std::{
    error::Error,
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{Span, field::Empty};
use tracing_datadog::{
    context::{TraceContextExt, TracingContextExt},
    http::W3CTraceContextHeaders,
};

/// Creates a span from a request.
fn make_span_from_request<B>(req: &Request<B>) -> Span {
    let http_method = req.method().as_str();
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
        "HTTP request",
        operation = "axum.request",
        resource = format!("{} {}", http_method, crate::http::path_group(req.uri().path())),
        http.base_url = req.headers().get(header::HOST).and_then(|h| h.to_str().ok()).or(req.uri().host()),
        http.method = %http_method,
        http.url = req.uri().path(),
        http.useragent = req.headers().get(header::USER_AGENT).and_then(|h| h.to_str().ok()),
        http.route = Empty,
        http.client.ip = client_ip,
        http.request_id = request_id,
        http.status_code = Empty,
        network.protocol.version = match req.version() {
            http::Version::HTTP_10 => "1.0",
            http::Version::HTTP_11 => "1.1",
            http::Version::HTTP_2 => "2.0",
            http::Version::HTTP_3 => "3.0",
            _ => "",
        },
        server.address = req.uri().host(),
        url.scheme = req.uri().scheme_str(),
        request_id = Empty,
        error.type = Empty,
        error.message = Empty,
        span.kind = "server",
        span.type = "web",

        // Our internal authentication claims
        auth.method = Empty,
        auth.user_uuid = Empty,
        auth.merchant_uuid = Empty,
        auth.account_uuid = Empty,
        auth.role = Empty,
        auth.api_version = Empty,

        // Datadog AppSec identity tags
        usr.id = Empty,
        usr.email = Empty,
        usr.session_id = Empty,
        usr.role = Empty,
        usr.merchant = Empty,
        usr.account = Empty,
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
/// # Examples
///
/// ```
/// # use axum::routing::{Router, get, post};
/// use komoju_datadog::axum::AxumTraceLayer;
///
/// # let router: Router<()> =
/// axum::Router::new()
///   // Example route that creates a span for each request.
///   .route("/sign_in", post(sign_in))
///   .layer(AxumTraceLayer)
///   // No traces on health checks.
///   .route("/health_check", get(health_check));
///
/// # async fn sign_in() {}
/// # async fn health_check() {}
/// ```
#[derive(Clone, Debug)]
pub struct AxumTraceLayer;

impl<S> Layer<S> for AxumTraceLayer {
    /// The wrapped service
    type Service = AxumTraceService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        AxumTraceService { inner }
    }
}

/// Middleware `Service` layer that creates OTel spans for every request.
#[derive(Debug, Clone)]
pub struct AxumTraceService<S> {
    /// The inner service layer.
    inner: S,
}

impl<S, B, B2> Service<Request<B>> for AxumTraceService<S>
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

            let method = req.method().as_str();
            let route = http_route(&req);

            if !route.is_empty() {
                span.record("resource", format!("{method} {route}").trim());
                span.record("http.route", route);
            }

            span.set_context(
                req.headers()
                    .extract_trace_context::<W3CTraceContextHeaders>(),
            );

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
