use crate::http::types;

use super::types::HttpError;

pub async fn not_found() -> types::HttpError {
    let headers = {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers
    };

    let body = types::ErrorResponseBody {
        message: "Not found".to_string(),
        error_code: "not_found".to_string(),
    };

    return HttpError {
        status: axum::http::StatusCode::NOT_FOUND,
        headers: Some(headers),
        body,
    };
}
