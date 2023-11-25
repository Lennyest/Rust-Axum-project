use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = std::result::Result<T, Error>; // What does this do? 

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    LoginFail,

    // -- Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExtensions,

    // -- Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}


impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExtensions => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            Self::TicketDeleteFailIdNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
        }
    }
}
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INT_RESP");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response();

        // Create a placeholder Axum response.

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the error into the response.
        response.extensions_mut().insert(self);

        response
    }
}