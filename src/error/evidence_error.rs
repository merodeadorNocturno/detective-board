// detective-board/src/error/evidence_error.rs
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum EvidenceError {
    NoEvidenceFound,
}

impl ResponseError for EvidenceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            EvidenceError::NoEvidenceFound => StatusCode::NOT_FOUND,
        }
    }
}
