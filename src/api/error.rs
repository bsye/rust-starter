use std::collections::BTreeMap;

use dioxus::fullstack::{AsStatusCode, ServerFnError, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldErrors(BTreeMap<String, String>);

impl FieldErrors {
    pub const FORM: &str = "form";

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
    #[error("Sign in to continue")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Something went wrong. Try again.")]
    Server,
}

impl ApiError {
    pub fn into_field_errors(self) -> FieldErrors {
        match self {
            ApiError::Invalid(errors) => errors,
            other => {
                let mut errors = FieldErrors::default();
                errors.add(FieldErrors::FORM, other.to_string());
                errors
            }
        }
    }
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
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
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

#[cfg(test)]
mod tests {
    use super::{ApiError, FieldErrors};

    #[test]
    fn empty_errors_are_ok() {
        assert_eq!(FieldErrors::default().into_result(), Ok(()));
    }

    #[test]
    fn other_errors_become_a_form_message() {
        let errors = ApiError::Server.into_field_errors();
        assert_eq!(
            errors.get(FieldErrors::FORM),
            Some("Something went wrong. Try again.")
        );
    }

    #[test]
    fn any_error_makes_the_input_invalid() {
        let mut errors = FieldErrors::default();
        errors.add("title", "Enter a title");
        assert_eq!(errors.clone().into_result(), Err(ApiError::Invalid(errors)));
    }
}
