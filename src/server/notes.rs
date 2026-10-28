use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Select, TransactionTrait,
};

use super::audit::{self, Action};
use super::auth::Actor;
use super::entities::note;
use crate::api::{ApiError, Note, NoteInput, NotePage};

pub const NOTES_PAGE_SIZE: u64 = 20;
const ENTITY: &str = "note";

impl From<note::Model> for Note {
    fn from(model: note::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            body: model.body,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

fn scoped(actor: Actor) -> Select<note::Entity> {
    note::Entity::find()
        .filter(note::Column::OwnerId.eq(actor.id()))
        .filter(note::Column::DeletedAt.is_null())
}

async fn find(db: &DatabaseConnection, actor: Actor, id: i64) -> Result<note::Model, ApiError> {
    scoped(actor)
        .filter(note::Column::Id.eq(id))
        .one(db)
        .await?
        .ok_or(ApiError::NotFound)
}

pub async fn list(db: &DatabaseConnection, actor: Actor, page: u64) -> Result<NotePage, DbErr> {
    let page = page.max(1);
    let paginator = scoped(actor)
        .order_by_desc(note::Column::Id)
        .paginate(db, NOTES_PAGE_SIZE);
    let counts = paginator.num_items_and_pages().await?;
    let items = paginator.fetch_page(page - 1).await?;
    Ok(NotePage {
        items: items.into_iter().map(Note::from).collect(),
        total: counts.number_of_items,
        page,
        page_count: counts.number_of_pages,
    })
}

pub async fn get(db: &DatabaseConnection, actor: Actor, id: i64) -> Result<Note, ApiError> {
    Ok(find(db, actor, id).await?.into())
}

pub async fn create(
    db: &DatabaseConnection,
    actor: Actor,
    input: NoteInput,
) -> Result<Note, ApiError> {
    let input = input.validated()?;
    let txn = db.begin().await?;
    let model = note::ActiveModel {
        owner_id: Set(actor.id()),
        title: Set(input.title),
        body: Set(input.body),
        ..Default::default()
    }
    .insert(&txn)
    .await?;
    audit::record(&txn, actor, Action::Created, ENTITY, [model.id]).await?;
    txn.commit().await?;
    Ok(model.into())
}

pub async fn update(
    db: &DatabaseConnection,
    actor: Actor,
    id: i64,
    input: NoteInput,
) -> Result<Note, ApiError> {
    let input = input.validated()?;
    let mut model = find(db, actor, id).await?.into_active_model();
    model.title = Set(input.title);
    model.body = Set(input.body);
    model.updated_at = Set(Utc::now());
    let txn = db.begin().await?;
    let model = model.update(&txn).await?;
    audit::record(&txn, actor, Action::Updated, ENTITY, [id]).await?;
    txn.commit().await?;
    Ok(model.into())
}

pub async fn delete(db: &DatabaseConnection, actor: Actor, id: i64) -> Result<(), ApiError> {
    let mut model = find(db, actor, id).await?.into_active_model();
    model.deleted_at = Set(Some(Utc::now()));
    let txn = db.begin().await?;
    model.update(&txn).await?;
    audit::record(&txn, actor, Action::Deleted, ENTITY, [id]).await?;
    txn.commit().await?;
    Ok(())
}

pub async fn insert_many(
    db: &DatabaseConnection,
    actor: Actor,
    inputs: Vec<NoteInput>,
) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    let models = inputs.into_iter().map(|input| note::ActiveModel {
        owner_id: Set(actor.id()),
        title: Set(input.title),
        body: Set(input.body),
        ..Default::default()
    });
    let ids: Vec<i64> = note::Entity::insert_many(models)
        .exec_with_returning(&txn)
        .await?
        .into_iter()
        .map(|model| model.id)
        .collect();
    audit::record(&txn, actor, Action::Created, ENTITY, ids).await?;
    txn.commit().await
}

#[cfg(test)]
#[path = "notes_tests.rs"]
mod tests;
