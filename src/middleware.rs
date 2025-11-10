use crate::config::Config;
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// Bearer token authentication middleware
pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if it starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Extract token
    let token = &auth_header[7..]; // Skip "Bearer "

    // Load config to get expected token
    let config = Config::from_env().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verify token
    if token != config.bearer_token {
        tracing::warn!("Invalid token attempt");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Token is valid, continue
    Ok(next.run(request).await)
}
