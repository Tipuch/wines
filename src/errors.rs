use actix_web::{error, http, HttpResponse};

#[derive(Fail, Debug)]
pub enum LoginError {
    #[fail(display = "Email or password is incorrect.")]
    ValidationError,
}

impl error::ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            LoginError::ValidationError => HttpResponse::new(http::StatusCode::BAD_REQUEST),
        }
    }
}
