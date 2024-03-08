use crate::http::types;

pub async fn not_found() -> types::ApiResponse<types::ErrorResponseBody> {
    let headers = {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers
    };

    let body = types::ErrorResponseBody {
        message: "Not found".to_string(),
        error_code: "not_found".to_string(),
    };

    return (
        axum::http::StatusCode::NOT_FOUND,
        headers,
        axum::response::Json(body),
    );
}
