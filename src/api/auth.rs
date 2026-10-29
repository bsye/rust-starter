use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::{ApiError, FieldErrors};
#[cfg(feature = "server")]
use {
    crate::analytics::{self, AnalyticsEvent},
    crate::server::auth::{self, Auth, accounts},
    dioxus::server::axum::Extension,
    sea_orm::DatabaseConnection,
};

pub const PASSWORD_MIN_CHARS: usize = 12;
pub const PASSWORD_MAX_CHARS: usize = 128;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub email: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

impl Credentials {
    pub const EMAIL: &str = "email";
    pub const PASSWORD: &str = "password";

    pub fn normalized(self) -> Self {
        Self {
            email: self.email.trim().to_lowercase(),
            password: self.password,
        }
    }

    pub fn validated_for_sign_up(self) -> Result<Self, ApiError> {
        let clean = self.normalized();
        let mut errors = FieldErrors::default();
        if !looks_like_email(&clean.email) {
            errors.add(Self::EMAIL, "Enter an email address like name@example.com");
        }
        let length = clean.password.chars().count();
        if length < PASSWORD_MIN_CHARS {
            errors.add(
                Self::PASSWORD,
                format!("Use at least {PASSWORD_MIN_CHARS} characters"),
            );
        } else if length > PASSWORD_MAX_CHARS {
            errors.add(
                Self::PASSWORD,
                format!("Use {PASSWORD_MAX_CHARS} characters or fewer"),
            );
        }
        errors.into_result()?;
        Ok(clean)
    }
}

fn looks_like_email(email: &str) -> bool {
    match email.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty()
                && !domain.contains('@')
                && domain.split('.').count() > 1
                && domain.split('.').all(|part| !part.is_empty())
                && !email.contains(char::is_whitespace)
        }
        None => false,
    }
}

#[post("/api/auth/sign-up", db: Extension<DatabaseConnection>, session: Auth)]
pub async fn sign_up(credentials: Credentials) -> Result<UserInfo, ApiError> {
    let user = accounts::create(&db, credentials).await?;
    auth::start_session(&session, user.id);
    analytics::capture(AnalyticsEvent::SignedUp, user.id);
    Ok(user)
}

#[post("/api/auth/sign-in", db: Extension<DatabaseConnection>, session: Auth)]
pub async fn sign_in(credentials: Credentials) -> Result<UserInfo, ApiError> {
    let user = accounts::verify(&db, credentials).await?;
    auth::start_session(&session, user.id);
    analytics::capture(AnalyticsEvent::SignedIn, user.id);
    Ok(user)
}

#[post("/api/auth/sign-out", session: Auth)]
pub async fn sign_out() -> Result<(), ApiError> {
    session.logout_user();
    Ok(())
}

pub fn request_user() -> Option<UserInfo> {
    #[cfg(feature = "server")]
    return auth::request_user();
    #[cfg(not(feature = "server"))]
    unreachable!("the browser reads the signed-in user from the server-rendered page")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn credentials(email: &str, password: &str) -> Credentials {
        Credentials {
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    #[test]
    fn normalizes_email_but_not_password() {
        let clean = credentials("  Ada@Example.COM ", " Secret ").normalized();
        assert_eq!(clean, credentials("ada@example.com", " Secret "));
    }

    #[test]
    fn accepts_plain_addresses() {
        for email in ["ada@example.com", "a.b+tag@mail.example.co.uk"] {
            assert!(looks_like_email(email), "{email}");
        }
    }

    #[test]
    fn rejects_malformed_addresses() {
        for email in [
            "",
            "ada",
            "@example.com",
            "ada@",
            "ada@example",
            "ada@@example.com",
            "a da@example.com",
            "ada@example..com",
        ] {
            assert!(!looks_like_email(email), "{email}");
        }
    }

    #[test]
    fn sign_up_requires_a_long_enough_password() {
        let error = credentials("ada@example.com", "short")
            .validated_for_sign_up()
            .unwrap_err();
        let ApiError::Invalid(errors) = error else {
            panic!("expected invalid input")
        };
        assert_eq!(
            errors.get(Credentials::PASSWORD),
            Some("Use at least 12 characters")
        );
    }

    #[test]
    fn sign_up_caps_password_length() {
        let long = "a".repeat(PASSWORD_MAX_CHARS + 1);
        assert!(
            credentials("ada@example.com", &long)
                .validated_for_sign_up()
                .is_err()
        );
    }
}
