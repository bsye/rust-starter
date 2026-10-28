use std::sync::LazyLock;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use crate::api::ApiError;

static DECOY_HASH: LazyLock<String> = LazyLock::new(|| {
    hash_now("decoy password for unknown emails").expect("hashing a fixed password succeeds")
});

pub async fn hash(password: String) -> Result<String, ApiError> {
    tokio::task::spawn_blocking(move || hash_now(&password))
        .await
        .map_err(|error| server_error(&error))?
        .map_err(|error| server_error(&error))
}

pub async fn verify(password: String, stored: Option<String>) -> bool {
    let user_exists = stored.is_some();
    let hash = stored.unwrap_or_else(|| DECOY_HASH.clone());
    let matches = tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash)
            .map(|parsed| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok()
            })
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false);
    user_exists && matches
}

fn hash_now(password: &str) -> Result<String, argon2::password_hash::Error> {
    Ok(Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string())
}

fn server_error(error: &dyn std::fmt::Display) -> ApiError {
    dioxus::logger::tracing::error!("password hashing failed: {error}");
    ApiError::Server
}

#[cfg(test)]
mod tests {
    use super::{hash, verify};

    #[tokio::test]
    async fn stores_an_argon2id_hash_not_the_password() {
        let stored = hash("correct horse battery".to_string()).await.unwrap();
        assert!(stored.starts_with("$argon2id$"));
        assert!(!stored.contains("correct horse battery"));
    }

    #[tokio::test]
    async fn verifies_only_the_right_password() {
        let stored = hash("correct horse battery".to_string()).await.unwrap();
        assert!(verify("correct horse battery".to_string(), Some(stored.clone())).await);
        assert!(!verify("wrong horse battery".to_string(), Some(stored)).await);
    }

    #[tokio::test]
    async fn unknown_user_never_verifies() {
        assert!(!verify("decoy password for unknown emails".to_string(), None).await);
    }

    #[tokio::test]
    async fn corrupt_stored_hash_does_not_verify() {
        assert!(!verify("anything".to_string(), Some("not a hash".to_string())).await);
    }
}
