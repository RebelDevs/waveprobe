use std::error::Error;

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

#[derive(Serialize, Debug)]
pub struct HttpErrorResponseBody {
    message: String,
    error: ErrorResponseBody,
}

#[derive(Debug)]
pub struct HttpError {
    pub status: axum::http::StatusCode,
    pub headers: Option<axum::http::HeaderMap>,
    pub body: ErrorResponseBody,
}

impl From<axum::extract::rejection::JsonRejection> for HttpError {
    fn from(rejection: axum::extract::rejection::JsonRejection) -> Self {
        let message = match rejection {
            axum::extract::rejection::JsonRejection::JsonDataError(e) => match e.source() {
                Some(source) => source.to_string(),
                None => "unknown".to_owned(),
            },
            _ => "unknown".to_owned(),
        };

        return Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            headers: None,
            body: ErrorResponseBody {
                error_code: "body_schema".to_owned(),
                message,
            },
        };
    }
}

impl axum::response::IntoResponse for HttpError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let payload = HttpErrorResponseBody {
            error: self.body.clone(),
            message: self.body.message,
        };

        let mut status = self.status;
        let payload_str;
        match serde_json::to_string(&payload) {
            Ok(s) => {
                payload_str = s;
            }
            Err(_) => {
                status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                payload_str = "Internal Server Error".to_owned();
            }
        };

        let headers = match self.headers {
            Some(h) => h,
            None => axum::http::HeaderMap::new(),
        };

        let mut response = axum::http::Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(axum::body::Body::from(payload_str))
            .unwrap();

        response.headers_mut().extend(headers);
        return response;
    }
}
