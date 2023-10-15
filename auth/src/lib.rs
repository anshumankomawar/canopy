use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, decode_header, DecodingKey};
use models::auth::Claims;
use tracing::debug;
use utils::error::Error;
use utils::error::Error::TokenError;

const AUTHORIZATION: &str = "Authorization";
const BEARER: &str = "Bearer ";

pub async fn authenticate_jwt<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, Error> {
    debug!("[jwt_authentication]\n");
    let authorization_header = match req.headers().get(AUTHORIZATION) {
        Some(header) => header,
        None => {
            return Err(Error::AuthError {
                error: "No Authorization Header".into(),
            })
        }
    };

    let authorization = match authorization_header.to_str() {
        Ok(authorization) => authorization,
        Err(_) => {
            return Err(Error::AuthError {
                error: "Authoriation Token Parse Error".into(),
            })
        }
    };

    if !authorization.starts_with(BEARER) {
        return Err(Error::AuthError {
            error: "Authorization Bearer Missing".into(),
        });
    }

    let jwt_token = authorization.trim_start_matches(BEARER);

    let token_header = match decode_header(&jwt_token) {
        Ok(token_header) => token_header,
        Err(e) => return Err(TokenError { error: e }),
    };

    debug!("token_header: {:#?}", token_header);

    let secret = "secret".as_bytes();
    let key = DecodingKey::from_secret(secret);

    let validation = jsonwebtoken::Validation::default();

    let token_data = match decode::<Claims>(jwt_token, &key, &validation) {
        Ok(claims) => claims,
        Err(e) => return Err(TokenError { error: e }),
    };

    let claims = token_data.claims;
    debug!("{:?}", claims);

    req.extensions_mut().insert(claims);

    return Ok(next.run(req).await);
}
