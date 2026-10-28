use dioxus::server::axum::{
    body::Body,
    extract::{Request, State},
    http::{Method, StatusCode, header::ORIGIN},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn require_same_origin(
    State(public_origin): State<String>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let reads_only = matches!(
        *request.method(),
        Method::GET | Method::HEAD | Method::OPTIONS
    );
    let origin = request
        .headers()
        .get(ORIGIN)
        .and_then(|value| value.to_str().ok());
    if reads_only || origin == Some(public_origin.as_str()) {
        next.run(request).await
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}

#[cfg(test)]
mod tests {
    use dioxus::server::axum::{Router, body::Body, http::Request, middleware, routing::post};
    use tower::ServiceExt;

    use super::*;

    const ORIGIN_URL: &str = "https://notes.example.com";

    async fn status(method: Method, origin: Option<&str>) -> StatusCode {
        let app = Router::new()
            .route("/write", post(|| async { "ok" }).get(|| async { "ok" }))
            .layer(middleware::from_fn_with_state(
                ORIGIN_URL.to_string(),
                require_same_origin,
            ));
        let mut request = Request::builder().method(method).uri("/write");
        if let Some(origin) = origin {
            request = request.header(ORIGIN, origin);
        }
        app.oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap()
            .status()
    }

    #[tokio::test]
    async fn allows_writes_from_our_origin() {
        assert_eq!(status(Method::POST, Some(ORIGIN_URL)).await, StatusCode::OK);
    }

    #[tokio::test]
    async fn blocks_writes_from_another_origin_or_none() {
        assert_eq!(
            status(Method::POST, Some("https://evil.example")).await,
            StatusCode::FORBIDDEN
        );
        assert_eq!(status(Method::POST, None).await, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn allows_reads_from_anywhere() {
        assert_eq!(status(Method::GET, None).await, StatusCode::OK);
    }
}
