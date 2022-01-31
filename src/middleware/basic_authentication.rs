use std::task::{Context, Poll};

use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::response::IntoResponse;
use axum::{body::Body, http::Request, response::Response};
use futures::future::BoxFuture;
use headers::authorization::Basic;
use headers::Authorization;
use tower::{Layer, Service};

use crate::errors::token::TokenError;
use crate::ProfileService;

#[derive(Clone)]
pub struct BasicAuthentication<S> {
    pub inner: S,
}

impl<S> Layer<S> for BasicAuthentication<S> {
    type Service = Self;

    fn layer(&self, inner: S) -> Self::Service {
        Self {
            inner,
        }
    }
}

impl<S> Service<Request<Body>> for BasicAuthentication<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = S::Response;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        // FIXME: Solve error handling within middleware
        Box::pin(async move {
            let mut request_parts = RequestParts::new(request);

            let profile_service = request_parts
                .extensions()
                .unwrap()
                .get::<ProfileService>()
                .unwrap()
                .clone();

            // FIXME: Handle invalid token.
            let TypedHeader(Authorization(basic)) =
                TypedHeader::<Authorization<Basic>>::from_request(&mut request_parts)
                    .await
                    .unwrap();

            let email = basic.username();
            let password = basic.password();

            let result = profile_service
                .verify_credentials(email, password)
                .await;

            if let Err(err) = result {
                return Ok(err.into_response());
            }

            let (is_valid, profile) = result.unwrap();
            if !is_valid {
                // FIXME: Not a token! Invalid basic credentials error
                return Ok(TokenError::InvalidToken.into_response());
            }

            // Inject Profile for further actions
            request_parts
                .extensions_mut()
                .unwrap()
                .insert(profile);

            let request = request_parts
                .try_into_request()
                .unwrap();

            let response = inner.call(request).await?;

            Ok(response)
        })
    }
}
