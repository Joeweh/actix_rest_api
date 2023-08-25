use std::env;
use std::future::{Ready, ready};
use actix_web::{Error as ActixWebError, FromRequest, HttpRequest};
use actix_web::dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::HeaderValue;
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::Error as JWTError;
use crate::utils::jwt::UserClaim;

pub struct AuthMiddlewareFactory;

// Transform "transforms" a service by wrapping it in another service.
// The Transform implementation's only job is to create new middleware instances that wrap other services.
impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;

    type Transform = AuthMiddleware<S>;

    // indicates an error that might occur when creating the middleware instance
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // new_transform creates a new instance of the middleware Service.
    // The created middleware should wrap the service indicated by the service parameter
    //
    // returns a Future to allow some asynchronous work to be done while creating the middleware.
    fn new_transform(&self, service: S) -> Self::Future {
        // We only need to create a new object, so we'll use a Ready future to wrap the new middleware inside a future.
        // This is similar to using Javascript's Promise.resolve to place a value inside a Promise
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
        S::Future: 'static,
        B: 'static, // B type parameter here represents the type of the body returned from the service
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    // Makes it easier to use an async block without needing to deal with the opaque future
    // types returned by async blocks.
    // LocalBoxFuture is the non-Send version of BoxFuture
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Actix calls poll_ready to determine if the service is ready to be invoked.
    // forward_ready! macro delegates this function to the wrapped service
    forward_ready!(service);

    // The call function is where all the "real" functionality goes.
    // You can inspect or mutate the request and response objects as needed,
    // and invoke the wrapped service if appropriate.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}

pub struct AuthToken {
    pub id: String
}

impl FromRequest for AuthToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        let authorization_header_option: Option<&HeaderValue> = req.headers().get(actix_web::http::header::AUTHORIZATION);

        // No Header was sent
        if authorization_header_option.is_none() { return ready(Err(ErrorUnauthorized("No authentication token sent!"))); }

        let authentication_token: String = authorization_header_option.unwrap().to_str().unwrap_or("").to_string().replace("Bearer ", "");

        // Couldn't convert Header::Authorization to String
        if authentication_token.is_empty() { return ready(Err(ErrorUnauthorized("Authentication token has foreign chars!"))) }

        let token_result: Result<TokenData<UserClaim>, JWTError> = decode::<UserClaim>(
            &authentication_token,
            &DecodingKey::from_secret(env::var("ACCESS_TOKEN_SECRET").unwrap().as_str().as_ref()),
            &Validation::default(),
        );

        match token_result {
            Ok(token) => ready(Ok(AuthToken { id: token.claims.user_id })),
            Err(_e) => ready(Err(ErrorUnauthorized("Invalid authentication token sent!"))),
        }
    }
}
