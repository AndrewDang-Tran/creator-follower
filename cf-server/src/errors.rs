use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use askama::Error as AskamaError;
use derive_more::{Display, Error};
use reqwest;
use std::convert::From;
use std::fmt;
use std::num::TryFromIntError;

#[derive(Debug, Display, Error)]
pub struct ErrorMessageWrapper {
    message: &'static str,
}

#[derive(Debug, Error)]
pub struct AnilistServerError {
    pub message: String,
    pub status_code: StatusCode,
}

impl fmt::Display for AnilistServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<AnilistServerError> for ServiceError {
    fn from(e: AnilistServerError) -> ServiceError {
        ServiceError::AnilistError(e)
    }
}

impl From<TryFromIntError> for ServiceError {
    fn from(_e: TryFromIntError) -> ServiceError {
        ServiceError::InternalError
    }
}

impl From<reqwest::Error> for ServiceError {
    fn from(e: reqwest::Error) -> ServiceError {
        if e.is_status() {
            if let Some(s) = e.status() {
                return ServiceError::AnilistError(AnilistServerError {
                    message: "Received error from AnilistServer".to_string(),
                    status_code: s,
                });
            }
        }
        ServiceError::InternalError
    }
}

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "An internal error occurred. Please try again later")]
    InternalError,

    #[display(fmt = "Unexpected Anilist data format: {}", _0)]
    AnilistDataFormat(ErrorMessageWrapper),

    #[display(fmt = "An internal error occurred: {}", _0)]
    InternalLogicError(ErrorMessageWrapper),

    #[display(fmt = "An error occurred in Anilist: {}", _0)]
    AnilistError(AnilistServerError),

    #[display(fmt = "An internal error occurred. Please try again later")]
    AskamaError(AskamaError),
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
            ServiceError::AnilistError(e) => e.status_code,
            ServiceError::AskamaError(_e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn anilist_data_format(message: &'static str) -> ServiceError {
    ServiceError::AnilistDataFormat(ErrorMessageWrapper { message })
}

pub fn internal_logic_error(message: &'static str) -> ServiceError {
    ServiceError::InternalLogicError(ErrorMessageWrapper { message })
}
