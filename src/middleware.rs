use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Get key from environment
    let expected_key = std::env::var("PCX_AUTH_KEY").map_err(|_| {
        log::error!("CRITICAL: PCX_AUTH_KEY environment variable is not set!");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 2. Extract header
    let auth_header = headers
        .get("PCX_AUTH_KEY")
        .and_then(|h| h.to_str().ok());

    // 3. Compare
    match auth_header {
        Some(key) if key == expected_key => {
            log::info!("✅ Auth Success: Valid key received.");
            // println!("✅ Auth Success: Valid key received.");
            Ok(next.run(request).await)
        }
        Some(key) => {
            log::error!("❌ Auth Failed: Incorrect key received. Expected: [{}], Got: [{}]", expected_key, key);
            // eprintln!("❌ Auth Failed: Incorrect key received. Expected: [{}], Got: [{}]", expected_key, key);
            Err(StatusCode::UNAUTHORIZED)
        }
        None => {
            log::error!("❌ Auth Failed: Missing PCX_AUTH_KEY header.");
            // eprintln!("❌ Auth Failed: Missing PCX_AUTH_KEY header.");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}