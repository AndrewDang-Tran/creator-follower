use actix_web::{
    error,
    http::{StatusCode, header::ContentType},
    HttpResponse,
};
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "An internal error occurred. Please try again Later")]
    InternalError,

    #[display(fmt = "Unexpected Anilist data format")]
    AnilistDataFormat,
}

impl error::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::AnilistDataFormat => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
