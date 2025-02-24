// detective-board/src/error/organization_error.rs
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum OrganizationError {
    NoOrganizationFound,
}

impl ResponseError for OrganizationError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            OrganizationError::NoOrganizationFound => StatusCode::NOT_FOUND,
        }
    }
}
