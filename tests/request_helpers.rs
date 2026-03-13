use anyhow::Result;
use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, Request, Response};
use http_body_util::BodyExt;
use opengame_api::routes::auth::login::LoginSuccessResponse;
use tower::ServiceExt;

pub async fn extract_bearer(response: Response<Body>) -> Result<String> {
    let body_bytes = response.into_body().collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let login_response: LoginSuccessResponse = serde_json::from_str(&body_str)?;
    Ok(login_response.bearer)
}

pub async fn send_request(
    app: &Router,
    method: &str,
    path: &str,
    headers: Option<HeaderMap>,
    body: Option<Body>,
) -> Result<Response<Body>> {
    let mut request_builder = Request::builder().method(method).uri(path);

    if let Some(headers) = headers {
        *request_builder.headers_mut().unwrap() = headers;
    }

    let request = request_builder.body(body.unwrap_or_else(Body::empty))?;
    let response = app.clone().oneshot(request).await?;
    Ok(response)
}

pub async fn login(app: &Router, credentials: &serde_json::Value) -> Result<Response<Body>> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    send_request(
        &app,
        "POST",
        "/auth/login",
        Some(headers),
        Some(Body::from(credentials.to_string())),
    )
    .await
}

pub async fn logout(app: &Router, bearer: &str) -> Result<Response<Body>> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {bearer}"))?,
    );

    send_request(app, "POST", "/auth/logout", Some(headers), None).await
}

pub async fn register(app: &Router, user: &serde_json::Value) -> Result<Response<Body>> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    send_request(
        &app,
        "POST",
        "/auth/register",
        Some(headers),
        Some(Body::from(user.to_string())),
    )
    .await
}
