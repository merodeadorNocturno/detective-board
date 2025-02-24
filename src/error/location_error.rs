// detective-board/src/error/location_error.rs
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum LocationError {
    NoLocationFound,
}

impl ResponseError for LocationError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            LocationError::NoLocationFound => StatusCode::NOT_FOUND,
        }
    }
}
