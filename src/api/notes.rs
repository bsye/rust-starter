use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::{ApiError, FieldErrors};
#[cfg(feature = "server")]
use {
    crate::server::{
        auth::{Auth, require_actor},
        notes,
    },
    dioxus::server::axum::Extension,
    sea_orm::DatabaseConnection,
};

pub const TITLE_MAX_CHARS: usize = 200;
pub const BODY_MAX_CHARS: usize = 10_000;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotePage {
    pub items: Vec<Note>,
    pub total: u64,
    pub page: u64,
    pub page_count: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NoteInput {
    pub title: String,
    pub body: String,
}

impl NoteInput {
    pub const TITLE: &str = "title";
    pub const BODY: &str = "body";

    pub fn validated(self) -> Result<Self, ApiError> {
        let title = self.title.trim().to_string();
        let body = self.body.trim_end().to_string();
        let mut errors = FieldErrors::default();
        if title.is_empty() {
            errors.add(Self::TITLE, "Enter a title");
        } else if title.chars().count() > TITLE_MAX_CHARS {
            errors.add(
                Self::TITLE,
                format!("Use {TITLE_MAX_CHARS} characters or fewer"),
            );
        }
        if body.chars().count() > BODY_MAX_CHARS {
            errors.add(
                Self::BODY,
                format!("Use {BODY_MAX_CHARS} characters or fewer"),
            );
        }
        errors.into_result()?;
        Ok(Self { title, body })
    }
}

#[get("/api/notes?page", db: Extension<DatabaseConnection>, session: Auth)]
pub async fn list_notes(page: u64) -> Result<NotePage, ApiError> {
    Ok(notes::list(&db, require_actor(&session)?, page).await?)
}

#[get("/api/notes/{id}", db: Extension<DatabaseConnection>, session: Auth)]
pub async fn get_note(id: i64) -> Result<Note, ApiError> {
    notes::get(&db, require_actor(&session)?, id).await
}

#[post("/api/notes", db: Extension<DatabaseConnection>, session: Auth)]
pub async fn create_note(input: NoteInput) -> Result<Note, ApiError> {
    notes::create(&db, require_actor(&session)?, input).await
}

#[put("/api/notes/{id}", db: Extension<DatabaseConnection>, session: Auth)]
pub async fn update_note(id: i64, input: NoteInput) -> Result<Note, ApiError> {
    notes::update(&db, require_actor(&session)?, id, input).await
}

#[delete("/api/notes/{id}", db: Extension<DatabaseConnection>, session: Auth)]
pub async fn delete_note(id: i64) -> Result<(), ApiError> {
    notes::delete(&db, require_actor(&session)?, id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(title: &str, body: &str) -> NoteInput {
        NoteInput {
            title: title.to_string(),
            body: body.to_string(),
        }
    }

    fn field_error(result: Result<NoteInput, ApiError>, field: &str) -> String {
        match result {
            Err(ApiError::Invalid(errors)) => errors.get(field).unwrap().to_string(),
            other => panic!("expected invalid input, got {other:?}"),
        }
    }

    #[test]
    fn trims_title_and_trailing_body_whitespace() {
        let clean = input("  Groceries  ", "  milk\n\n").validated().unwrap();
        assert_eq!(clean, input("Groceries", "  milk"));
    }

    #[test]
    fn rejects_blank_title() {
        let message = field_error(input("   ", "body").validated(), NoteInput::TITLE);
        assert_eq!(message, "Enter a title");
    }

    #[test]
    fn rejects_title_over_the_limit() {
        let long = "a".repeat(TITLE_MAX_CHARS + 1);
        let message = field_error(input(&long, "").validated(), NoteInput::TITLE);
        assert_eq!(message, "Use 200 characters or fewer");
    }

    #[test]
    fn counts_characters_not_bytes() {
        let title = "é".repeat(TITLE_MAX_CHARS);
        assert!(input(&title, "").validated().is_ok());
    }

    #[test]
    fn rejects_body_over_the_limit() {
        let long = "a".repeat(BODY_MAX_CHARS + 1);
        let message = field_error(input("Title", &long).validated(), NoteInput::BODY);
        assert_eq!(message, "Use 10000 characters or fewer");
    }
}
