use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    BoxError,
    Json,
};
use http_body::Body;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::internal::ServerError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(request_parts: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(object) = Json::<T>::from_request(request_parts).await?;
        object.validate()?;
        Ok(Self(object))
    }
}
