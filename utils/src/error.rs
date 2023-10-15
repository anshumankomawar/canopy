use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

/*
 * `Error` represents the custom error type for this module, encompassing various error scenarios.
 *
 * Variants:
 * - `HashError`: Represents an error encountered during hashing operations.
 * - `WrongPassword`: Represents an error indicating that the provided password is incorrect.
 * - `QueryError`: Represents an error encountered during a database query.
 *      - `error` [sqlx::Error]: The underlying SQLx error.
 * - `AuthError`: Represents an authentication-related error.
 *      - `error` [String]: The specific error message describing the authentication error.
 * - `TokenError`: Represents an error related to token processing.
 *      - `error` [jsonwebtoken::errors::Error]: The underlying JSON Web Token (JWT) error.
 */
#[derive(Debug)]
pub enum Error {
    HashError,
    WrongPassword,
    QueryError { error: sqlx::Error },
    AuthError { error: String },
    TokenError { error: jsonwebtoken::errors::Error },
}

#[derive(Debug)]
pub enum ClientError {
    IncorrectPassword,
    HashError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("Error: {self:?}");
        match self {
            Error::WrongPassword => (
                StatusCode::UNAUTHORIZED,
                format!("{:?}", ClientError::IncorrectPassword),
            )
                .into_response(),
            Error::HashError => (
                StatusCode::UNAUTHORIZED,
                format!("{:?}", ClientError::HashError),
            )
                .into_response(),
            Error::QueryError { error } => {
                (StatusCode::UNAUTHORIZED, format!("QueryError: {:?}", error)).into_response()
            }
            Error::TokenError { error } => {
                (StatusCode::UNAUTHORIZED, format!("TokenError: {:?}", error)).into_response()
            }
            Error::AuthError { error } => {
                (StatusCode::UNAUTHORIZED, format!("AuthError: {:?}", error)).into_response()
            }
        }
    }
}
