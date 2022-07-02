use actix_web::{
    error,
    http::{StatusCode, header::ContentType},
    HttpResponse,
};
use std::error::Error as StdError;
use derive_more::{Display, Error};

#[derive(Debug, Display)]
pub struct ErrorMessageWrapper {
    message: &'static str
}

impl StdError for ErrorMessageWrapper {}


#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "An internal error occurred. Please try again Later")]
    InternalError,

    #[display(fmt = "Unexpected Anilist data format: {}", _0)]
    AnilistDataFormat(ErrorMessageWrapper),

    #[display(fmt = "An internal error occurred: {}", _0)]
    InternalLogicError(ErrorMessageWrapper)
}

impl error::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match &self {
            ServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::AnilistDataFormat(_e) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::InternalLogicError(_e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn anilist_data_format(message: &'static str) ->  ServiceError {
    ServiceError::AnilistDataFormat(ErrorMessageWrapper { message })
}

pub fn internal_logic_error(message: &'static str) -> ServiceError {
    ServiceError::InternalLogicError(ErrorMessageWrapper { message })
}
