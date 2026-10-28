use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    SqlErr,
};

use super::password;
use crate::api::{ApiError, Credentials, FieldErrors, UserInfo};
use crate::server::entities::user;

const WRONG_CREDENTIALS: &str = "Email or password is wrong";
const EMAIL_TAKEN: &str = "An account with this email already exists";

pub async fn create(
    db: &DatabaseConnection,
    credentials: Credentials,
) -> Result<UserInfo, ApiError> {
    let credentials = credentials.validated_for_sign_up()?;
    let password_hash = password::hash(credentials.password).await?;
    let inserted = user::ActiveModel {
        email: Set(credentials.email),
        password_hash: Set(password_hash),
        ..Default::default()
    }
    .insert(db)
    .await;
    match inserted {
        Ok(model) => Ok(UserInfo {
            id: model.id,
            email: model.email,
        }),
        Err(error) if matches!(error.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            Err(invalid(Credentials::EMAIL, EMAIL_TAKEN))
        }
        Err(error) => Err(error.into()),
    }
}

pub async fn verify(
    db: &DatabaseConnection,
    credentials: Credentials,
) -> Result<UserInfo, ApiError> {
    let credentials = credentials.normalized();
    let found = user::Entity::find()
        .filter(user::Column::Email.eq(&credentials.email))
        .one(db)
        .await?;
    let stored = found.as_ref().map(|model| model.password_hash.clone());
    let password_ok = password::verify(credentials.password, stored).await;
    match found {
        Some(model) if password_ok => Ok(UserInfo {
            id: model.id,
            email: model.email,
        }),
        _ => Err(invalid(FieldErrors::FORM, WRONG_CREDENTIALS)),
    }
}

fn invalid(field: &str, message: &str) -> ApiError {
    let mut errors = FieldErrors::default();
    errors.add(field, message);
    ApiError::Invalid(errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::db::test_db;

    fn credentials(email: &str, password: &str) -> Credentials {
        Credentials {
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    const PASSWORD: &str = "correct horse battery";

    #[sqlx::test(migrations = false)]
    async fn creates_an_account_with_a_normalized_email(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        let user = create(&db, credentials(" Ada@Example.com ", PASSWORD))
            .await
            .unwrap();
        assert_eq!(user.email, "ada@example.com");
    }

    #[sqlx::test(migrations = false)]
    async fn rejects_a_duplicate_email_in_any_case(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        create(&db, credentials("ada@example.com", PASSWORD))
            .await
            .unwrap();
        let error = create(&db, credentials("ADA@example.com", PASSWORD))
            .await
            .unwrap_err();
        assert_eq!(error, invalid(Credentials::EMAIL, EMAIL_TAKEN));
        assert_eq!(user::Entity::find().all(&db).await.unwrap().len(), 1);
    }

    #[sqlx::test(migrations = false)]
    async fn signs_in_with_the_right_password(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        let created = create(&db, credentials("ada@example.com", PASSWORD))
            .await
            .unwrap();
        let signed_in = verify(&db, credentials("ADA@example.com ", PASSWORD))
            .await
            .unwrap();
        assert_eq!(signed_in, created);
    }

    #[sqlx::test(migrations = false)]
    async fn wrong_password_and_unknown_email_get_the_same_answer(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        create(&db, credentials("ada@example.com", PASSWORD))
            .await
            .unwrap();
        let wrong_password =
            verify(&db, credentials("ada@example.com", "wrong horse battery")).await;
        let unknown_email = verify(&db, credentials("bob@example.com", PASSWORD)).await;
        assert_eq!(
            wrong_password,
            Err(invalid(FieldErrors::FORM, WRONG_CREDENTIALS))
        );
        assert_eq!(unknown_email, wrong_password);
    }
}
