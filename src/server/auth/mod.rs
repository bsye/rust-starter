pub mod accounts;
mod password;

use anyhow::Context;
use async_trait::async_trait;
use axum_session::{SameSite, SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use axum_session_sqlx::SessionPgPool;
use dioxus::fullstack::FullstackContext;
use dioxus::server::axum::Router;
use sea_orm::{DatabaseConnection, EntityTrait, SqlxPostgresConnector, sqlx::PgPool};

use super::{config::Config, entities::user};
use crate::api::{ApiError, UserInfo};

pub type Auth = AuthSession<User, i64, SessionPgPool, PgPool>;

const SESSION_TABLE: &str = "sessions";
const SESSION_COOKIE: &str = "session";

#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
        }
    }
}

#[async_trait]
impl Authentication<User, i64, PgPool> for User {
    async fn load_user(id: i64, pool: Option<&PgPool>) -> anyhow::Result<User> {
        let pool = pool.context("the auth layer has no database pool")?;
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());
        let model = user::Entity::find_by_id(id)
            .one(&db)
            .await?
            .context("user no longer exists")?;
        Ok(User {
            id: model.id,
            email: model.email,
        })
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actor(i64);

impl Actor {
    pub fn id(self) -> i64 {
        self.0
    }

    pub(super) fn trusted(id: i64) -> Self {
        Actor(id)
    }
}

pub fn require_actor(session: &Auth) -> Result<Actor, ApiError> {
    session
        .current_user
        .as_ref()
        .map(|user| Actor(user.id))
        .ok_or(ApiError::Unauthorized)
}

pub fn request_user() -> Option<UserInfo> {
    let context = FullstackContext::current().expect("request_user runs while rendering a request");
    let session = context
        .extension::<Auth>()
        .expect("the auth session layer is installed");
    session.current_user.map(UserInfo::from)
}

pub fn start_session(session: &Auth, user_id: i64) {
    session.session.renew();
    session.login_user(user_id);
    session.remember_user(true);
}

pub async fn layers(
    router: Router,
    db: &DatabaseConnection,
    config: &Config,
) -> anyhow::Result<Router> {
    let pool = db.get_postgres_connection_pool().clone();
    let session_config = SessionConfig::default()
        .with_table_name(SESSION_TABLE)
        .with_session_name(SESSION_COOKIE)
        .with_secure(config.secure_cookies())
        .with_http_only(true)
        .with_cookie_same_site(SameSite::Lax);
    let store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config).await?;
    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(None);
    Ok(router
        .layer(
            AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(pool))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(store)))
}

#[cfg(test)]
pub async fn test_actor(db: &DatabaseConnection, email: &str) -> Actor {
    use sea_orm::{ActiveModelTrait, ActiveValue::Set};
    let model = user::ActiveModel {
        email: Set(email.to_string()),
        password_hash: Set("not a real hash".to_string()),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();
    Actor(model.id)
}
