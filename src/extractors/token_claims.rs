use axum::extract::Extension;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};

use crate::core::errors::ServerError;
use crate::models::claims::Claims;
use crate::TokenService;

#[derive(Debug, Clone, Copy, Default)]
pub struct TokenClaims<T>(pub T);

#[async_trait]
impl<B> FromRequest<B> for TokenClaims<Claims>
where
    B: Send,
{
    type Rejection = ServerError;

    async fn from_request(request_parts: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request(request_parts)
            .await
            .unwrap();

        let Extension(token_service) = Extension::<TokenService>::from_request(request_parts)
            .await
            .unwrap();

        let claims = token_service.decode(bearer.token());
        Ok(Self(claims))
    }
}
