use std::task::{Context, Poll};

use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::{body::Body, http::Request, response::Response};
use futures::future::BoxFuture;
use headers::authorization::Bearer;
use headers::Authorization;
use tower::{Layer, Service};
use tracing::{debug, info};

use crate::{ProfileService, TokenService};

#[derive(Debug, Clone)]
pub struct TokenAuthenticationLayer;

impl Default for TokenAuthenticationLayer {
    fn default() -> Self {
        Self
    }
}

impl<S> Layer<S> for TokenAuthenticationLayer {
    type Service = TokenAuthentication<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TokenAuthentication {
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenAuthentication<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for TokenAuthentication<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = S::Response;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        // FIXME: Solve error handling within middleware
        Box::pin(async move {
            let mut request_parts = RequestParts::new(request);

            // FIXME: Maybe create a helper function to grab services from extensions, this is very verbose.
            let token_service = request_parts
                .extensions()
                .unwrap()
                .get::<TokenService>()
                .unwrap()
                .clone();

            let profile_service = request_parts
                .extensions()
                .unwrap()
                .get::<ProfileService>()
                .unwrap()
                .clone();

            // FIXME: Handle invalid token.
            let TypedHeader(Authorization(bearer)) =
                TypedHeader::<Authorization<Bearer>>::from_request(&mut request_parts)
                    .await
                    .unwrap();

            let token = bearer.token();
            debug!("Bearer Token: {token:?}");

            let claims = token_service.decode(token).unwrap();
            info!("Claims: {claims:?}");

            let profile_id = claims.sub;
            let profile = profile_service
                .read(profile_id)
                .await
                .unwrap();

            // Inject Profile and Claims
            request_parts
                .extensions_mut()
                .unwrap()
                .insert(profile);

            request_parts
                .extensions_mut()
                .unwrap()
                .insert(claims);

            let request = request_parts
                .try_into_request()
                .unwrap();

            let response = inner.call(request).await?;

            Ok(response)
        })
    }
}
