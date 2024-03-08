use serde::{Deserialize, Serialize};

pub type ApiResponse<T> = (
    axum::http::StatusCode,
    axum::http::HeaderMap,
    axum::response::Json<T>,
);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ErrorResponseBody {
    pub message: String,
    pub error_code: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HttpError {
    expose: bool,
    body: ErrorResponseBody,
}
