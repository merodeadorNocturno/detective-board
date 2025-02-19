use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;

#[derive(Debug, Display)]
pub enum PersonError {
    NoPersonFound,
    PersonCreationFailure,
}

impl ResponseError for PersonError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            PersonError::NoPersonFound => StatusCode::NOT_FOUND,
            PersonError::PersonCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
