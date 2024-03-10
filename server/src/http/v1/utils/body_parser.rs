use crate::http::types::HttpError;
use axum::response::IntoResponse;
use axum_macros::FromRequest;
use serde::Serialize;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(HttpError))]
pub struct JsonBody<T>(pub T);

impl<T: Serialize> IntoResponse for JsonBody<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}
