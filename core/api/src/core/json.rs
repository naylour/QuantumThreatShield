use axum::{Json, extract::FromRequest, http::StatusCode};
use serde::de::DeserializeOwned;

use super::response::ApiResponse;

pub struct ApiJson<T>(pub T);

impl<S, T> FromRequest<S> for ApiJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiResponse<()>;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(ApiJson(value)),
            Err(error) => Err(ApiResponse::error(
                StatusCode::UNPROCESSABLE_ENTITY,
                Some("Невалидный JSON"),
                Some(vec![error.to_string()]),
            )),
        }
    }
}
