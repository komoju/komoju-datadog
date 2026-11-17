//! `OpenTelemetry` http_server helper functions.
//!
//! Adapted from <https://github.com/will-bank/datadog-tracing>.

use axum::extract::{ConnectInfo, MatchedPath};
use http::{HeaderMap, Method, Request, Response, Uri, Version};
use pin_project_lite::pin_project;
use std::{
    error::Error,
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::Span;
use tracing_datadog::{
    context::{TraceContextExt, TracingContextExt},
    http::W3CTraceContextHeaders,
};

#[doc(hidden)]
pub use http as __http;
#[doc(hidden)]
pub use tracing as __tracing;

/// The parts of an incoming request that the request span describes.
///
/// This is what the span factory of [`AxumTraceLayer`] receives.
#[derive(Debug)]
pub struct RequestInfo<'a> {
    /// The request method.
    pub method: &'a Method,
    /// The request URI.
    pub uri: &'a Uri,
    /// The request headers.
    pub headers: &'a HeaderMap,
    /// The HTTP version of the request.
    pub version: Version,
    /// The client IP, taken from `X-Forwarded-For` or the connection peer.
    pub client_ip: Option<String>,
}

/// Creates the request span, including application-specific tags.
///
/// [`AxumTraceLayer`] records `resource`, `http.route` and `http.status_code` on the span it is
/// given, so those fields must be declared, and every tag the application records later must be
/// declared here too. `tracing` silently drops writes to undeclared fields.
///
/// # Examples
///
/// ```
/// use komoju_datadog::axum::{AxumTraceLayer, RequestInfo};
/// use komoju_datadog::request_span;
/// use tracing::Span;
///
/// // Declares the standard request fields plus the application's own `tenant.id` tag,
/// // which can then be recorded from anywhere inside the request.
/// fn make_span(info: &RequestInfo<'_>) -> Span {
///     request_span!(info, tenant.id)
/// }
///
/// # let router: axum::Router<()> = axum::Router::new().layer(AxumTraceLayer::with_make_span(make_span));
/// ```
#[macro_export]
macro_rules! request_span {
    ($info:expr $(, $($tag:ident).+)* $(,)?) => {{
        let info = $info;
        $crate::axum::__tracing::info_span!(
            "HTTP request",
            operation = "axum.request",
            resource = ::std::format!("{} {}", info.method, $crate::http::path_group(info.uri.path())),
            http.base_url = info.headers.get($crate::axum::__http::header::HOST).and_then(|value| value.to_str().ok()).or(info.uri.host()),
            http.method = %info.method,
            http.url = info.uri.path(),
            http.useragent = info.headers.get($crate::axum::__http::header::USER_AGENT).and_then(|value| value.to_str().ok()),
            http.route = $crate::axum::__tracing::field::Empty,
            http.client.ip = info.client_ip.as_deref(),
            http.request_id = info.headers.get("X-Request-Id").and_then(|value| value.to_str().ok()),
            http.status_code = $crate::axum::__tracing::field::Empty,
            network.protocol.version = match info.version {
                $crate::axum::__http::Version::HTTP_10 => "1.0",
                $crate::axum::__http::Version::HTTP_11 => "1.1",
                $crate::axum::__http::Version::HTTP_2 => "2.0",
                $crate::axum::__http::Version::HTTP_3 => "3.0",
                _ => "",
            },
            server.address = info.uri.host(),
            url.scheme = info.uri.scheme_str(),
            request_id = $crate::axum::__tracing::field::Empty,
            error.type = $crate::axum::__tracing::field::Empty,
            error.message = $crate::axum::__tracing::field::Empty,
            otel.status_code = $crate::axum::__tracing::field::Empty,
            span.kind = "server",
            span.type = "web",

            // Datadog AppSec identity tags
            usr.id = $crate::axum::__tracing::field::Empty,
            usr.email = $crate::axum::__tracing::field::Empty,
            usr.session_id = $crate::axum::__tracing::field::Empty,
            usr.role = $crate::axum::__tracing::field::Empty,
            usr.merchant = $crate::axum::__tracing::field::Empty,
            usr.account = $crate::axum::__tracing::field::Empty,

            // Application-specific tags, declared by the caller.
            $($($tag).+ = $crate::axum::__tracing::field::Empty,)*
        )
    }};
}

/// Creates the request span with only the standard fields.
fn standard_span(info: &RequestInfo<'_>) -> Span {
    crate::request_span!(info)
}

/// Axum Layer to create OTel spans for requests.
///
/// Tags beyond the standard request fields are application-specific, so they are declared by the
/// span factory instead of this layer. See [`request_span!`].
///
/// # Examples
///
/// ```
/// use komoju_datadog::axum::{AxumTraceLayer, RequestInfo};
/// use komoju_datadog::request_span;
/// use tracing::Span;
///
/// fn make_span(info: &RequestInfo<'_>) -> Span {
///     request_span!(info, tenant.id)
/// }
///
/// # let router: axum::Router<()> =
/// axum::Router::new()
///   // Example route that creates a span for each request.
///   .route("/sign_in", axum::routing::post(sign_in))
///   .layer(AxumTraceLayer::with_make_span(make_span))
///   // No traces on health checks.
///   .route("/health_check", axum::routing::get(health_check));
///
/// # async fn sign_in() {}
/// # async fn health_check() {}
/// ```
///
/// Omitting the factory uses the standard request span:
///
/// ```
/// # let router: axum::Router<()> = axum::Router::new().layer(komoju_datadog::axum::AxumTraceLayer::default());
/// ```
#[derive(Clone)]
pub struct AxumTraceLayer<F = fn(&RequestInfo<'_>) -> Span> {
    make_span: F,
}

impl Default for AxumTraceLayer {
    /// Creates a layer using the standard request span, without application-specific tags.
    fn default() -> Self {
        Self {
            make_span: standard_span,
        }
    }
}

impl AxumTraceLayer {
    /// Creates a layer using the standard request span, without application-specific tags.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<F> AxumTraceLayer<F> {
    /// Creates a layer using `make_span` to create the request span.
    ///
    /// Use [`request_span!`] to declare the standard fields together with the application's tags.
    pub fn with_make_span(make_span: F) -> Self
    where
        F: for<'a> Fn(&RequestInfo<'a>) -> Span,
    {
        Self { make_span }
    }
}

impl<S, F> Layer<S> for AxumTraceLayer<F>
where
    F: Clone,
{
    /// The wrapped service
    type Service = AxumTraceService<S, F>;
    fn layer(&self, inner: S) -> Self::Service {
        AxumTraceService {
            inner,
            make_span: self.make_span.clone(),
        }
    }
}

/// Middleware `Service` layer that creates OTel spans for every request.
#[derive(Debug, Clone)]
pub struct AxumTraceService<S, F> {
    /// The inner service layer.
    inner: S,
    /// Creates the request span.
    make_span: F,
}

impl<S, F, B, B2> Service<Request<B>> for AxumTraceService<S, F>
where
    S: Service<Request<B>, Response = Response<B2>> + Clone + Send + 'static,
    S::Error: Error + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
    F: for<'a> Fn(&RequestInfo<'a>) -> Span,
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
            let span = (self.make_span)(&request_info(&req));

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

/// Extracts the parts of a request that the request span describes.
fn request_info<B>(request: &Request<B>) -> RequestInfo<'_> {
    let client_ip = request
        .headers()
        .get("X-Forwarded-For")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
        .or_else(|| {
            request
                .extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|addr| addr.ip().to_string())
        });

    RequestInfo {
        method: request.method(),
        uri: request.uri(),
        headers: request.headers(),
        version: request.version(),
        client_ip,
    }
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

/// Returns the route that matched a request, or an empty string.
#[inline]
fn http_route<B>(req: &Request<B>) -> &str {
    req.extensions()
        .get::<MatchedPath>()
        .map_or_else(|| "", |mp| mp.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, routing::get};
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;
    use tracing::span::{Attributes, Record};
    use tracing::{Event, Id, Subscriber};
    use tracing_subscriber::{
        Layer as SubscriberLayer, layer::Context as SubscriberContext, layer::SubscriberExt,
        registry::LookupSpan,
    };

    /// Records the fields written to every span.
    #[derive(Clone, Default)]
    struct Capture(Arc<Mutex<Vec<String>>>);

    impl Capture {
        fn fields(&self) -> Vec<String> {
            self.0.lock().unwrap().clone()
        }
    }

    impl<S> SubscriberLayer<S> for Capture
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        fn on_new_span(&self, attrs: &Attributes<'_>, _id: &Id, _ctx: SubscriberContext<'_, S>) {
            let mut fields = Fields(Vec::new());
            attrs.record(&mut fields);
            self.0.lock().unwrap().extend(fields.0);
        }

        fn on_record(&self, _id: &Id, values: &Record<'_>, _ctx: SubscriberContext<'_, S>) {
            let mut fields = Fields(Vec::new());
            values.record(&mut fields);
            self.0.lock().unwrap().extend(fields.0);
        }

        fn on_event(&self, _event: &Event<'_>, _ctx: SubscriberContext<'_, S>) {}
    }

    struct Fields(Vec<String>);

    impl tracing::field::Visit for Fields {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.0.push(format!("{}={value}", field.name()));
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.0.push(format!("{}={value:?}", field.name()));
        }
    }

    /// The standard span, plus an application-specific tag.
    fn tagged_span(info: &RequestInfo<'_>) -> Span {
        crate::request_span!(info, tenant.id)
    }

    #[tokio::test]
    async fn custom_tags_are_recorded_on_the_request_span() {
        let capture = Capture::default();
        let _guard =
            tracing::subscriber::set_default(tracing_subscriber::registry().with(capture.clone()));
        let app = Router::new()
            .route(
                "/",
                get(async || {
                    Span::current().record("tenant.id", "tenant42");
                    "ok"
                }),
            )
            .layer(AxumTraceLayer::with_make_span(tagged_span));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        assert!(
            capture.fields().contains(&"tenant.id=tenant42".to_owned()),
            "the declared tag should be recorded: {:?}",
            capture.fields()
        );
    }

    /// `update_span_from_error` records these, so they must be declared or error spans lose them.
    #[tokio::test]
    async fn error_spans_are_tagged() {
        #[derive(Debug)]
        struct Failure;

        impl std::fmt::Display for Failure {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("upstream exploded")
            }
        }

        impl Error for Failure {}

        let capture = Capture::default();
        let _guard =
            tracing::subscriber::set_default(tracing_subscriber::registry().with(capture.clone()));
        let service = AxumTraceService {
            inner: tower::service_fn(|_request: Request<Body>| async {
                Err::<Response<Body>, Failure>(Failure)
            }),
            make_span: standard_span,
        };

        let _ = service
            .oneshot(Request::builder().uri("/fail").body(Body::empty()).unwrap())
            .await;

        let fields = capture.fields();
        assert!(
            fields.contains(&"otel.status_code=ERROR".to_owned()),
            "error spans must be tagged: {fields:?}"
        );
        assert!(
            fields.iter().any(|field| field.starts_with("error.type=")),
            "error spans must carry the error type: {fields:?}"
        );
    }

    #[tokio::test]
    async fn the_standard_span_declares_the_fields_the_layer_records() {
        let capture = Capture::default();
        let _guard =
            tracing::subscriber::set_default(tracing_subscriber::registry().with(capture.clone()));
        let app = Router::new()
            .route("/items", get(async || "ok"))
            .layer(AxumTraceLayer::default());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/items")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let fields = capture.fields();
        for field in [
            "resource=GET /items",
            "http.route=/items",
            "http.method=GET",
            "http.status_code=200",
        ] {
            assert!(fields.contains(&field.to_owned()), "{field}: {fields:?}");
        }
    }
}
