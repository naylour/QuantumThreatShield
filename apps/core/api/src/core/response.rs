use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug)]
pub enum ApiResponseStatus {
    Success,
    Error,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ApiResponse<T: Serialize> {
    pub status: ApiResponseStatus,

    pub code: u16,

    pub message: &'static str,

    pub reasons: Option<Vec<String>>,

    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(code: StatusCode, data: T, message: Option<&'static str>) -> Self {
        let message = message.unwrap_or_else(|| code.canonical_reason().unwrap_or("OK"));

        Self {
            status: ApiResponseStatus::Success,
            code: code.as_u16(),
            message,
            reasons: None,
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn error(
        code: StatusCode,
        message: Option<&'static str>,
        reasons: Option<Vec<String>>,
    ) -> Self {
        let message = message.unwrap_or_else(|| code.canonical_reason().unwrap_or("OK"));

        Self {
            status: ApiResponseStatus::Error,
            code: code.as_u16(),
            message,
            reasons,
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status, Json(self)).into_response()
    }
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<()>>;
