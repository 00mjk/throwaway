use std::fmt::Debug;

use axum::extract::Extension;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use tracing::{debug, info};

use crate::errors::internal::ServerError;
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
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(request_parts).await?;

        let Extension(token_service) = Extension::<TokenService>::from_request(request_parts).await?;

        let token = bearer.token();
        debug!("Token: {token:?}");

        let claims = token_service.decode(token)?;
        info!("Claims: {claims:?}");

        Ok(Self(claims))
    }
}
