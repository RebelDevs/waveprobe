use crate::http::types;
use axum::extract::{Json, Path};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct PathParams {
    id: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseBody {
    pub id: String,
}

pub async fn get_result(Path(params): Path<PathParams>) -> types::ApiResponse<ResponseBody> {
    return (
        axum::http::StatusCode::OK,
        axum::http::HeaderMap::new(),
        Json(ResponseBody { id: params.id }),
    );
}
