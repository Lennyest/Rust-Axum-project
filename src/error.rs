use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = std::result::Result<T, Error>; // What does this do? 

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // -- Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // -- Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}


impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INT_RESP");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}