use std::collections::BTreeMap;

use dioxus::fullstack::{AsStatusCode, ServerFnError, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldErrors(BTreeMap<String, String>);

impl FieldErrors {
    pub fn add(&mut self, field: &str, message: impl Into<String>) {
        self.0.insert(field.to_string(), message.into());
    }

    pub fn get(&self, field: &str) -> Option<&str> {
        self.0.get(field).map(String::as_str)
    }

    pub fn into_result(self) -> Result<(), ApiError> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(ApiError::Invalid(self))
        }
    }
}

#[derive(thiserror::Error, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Check the highlighted fields")]
    Invalid(FieldErrors),
    #[error("Not found")]
    NotFound,
    #[error("Something went wrong. Try again.")]
    Server,
}

impl From<ServerFnError> for ApiError {
    fn from(_: ServerFnError) -> Self {
        ApiError::Server
    }
}

impl AsStatusCode for ApiError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            ApiError::Invalid(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Server => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl From<sea_orm::DbErr> for ApiError {
    fn from(error: sea_orm::DbErr) -> Self {
        dioxus::logger::tracing::error!("database error: {error}");
        ApiError::Server
    }
}
