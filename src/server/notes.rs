use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryOrder,
};

use super::entities::note;
use crate::api::{ApiError, Note, NoteInput, NotePage};

pub const NOTES_PAGE_SIZE: u64 = 20;

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

pub async fn list(db: &DatabaseConnection, page: u64) -> Result<NotePage, DbErr> {
    let page = page.max(1);
    let paginator = note::Entity::find()
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

pub async fn get(db: &DatabaseConnection, id: i64) -> Result<Note, ApiError> {
    note::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Note::from)
        .ok_or(ApiError::NotFound)
}

pub async fn create(db: &DatabaseConnection, input: NoteInput) -> Result<Note, ApiError> {
    let input = input.validated()?;
    let model = note::ActiveModel {
        title: Set(input.title),
        body: Set(input.body),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(model.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::db::test_db;

    fn input(title: &str) -> NoteInput {
        NoteInput {
            title: title.to_string(),
            body: String::new(),
        }
    }

    #[sqlx::test(migrations = false)]
    async fn lists_newest_first_with_totals(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        for n in 1..=NOTES_PAGE_SIZE + 1 {
            create(&db, input(&format!("Note {n}"))).await.unwrap();
        }
        let first = list(&db, 1).await.unwrap();
        assert_eq!(first.items.len() as u64, NOTES_PAGE_SIZE);
        assert_eq!(
            first.items[0].title,
            format!("Note {}", NOTES_PAGE_SIZE + 1)
        );
        assert_eq!((first.total, first.page_count), (NOTES_PAGE_SIZE + 1, 2));
        let second = list(&db, 2).await.unwrap();
        assert_eq!(second.items.len(), 1);
    }

    #[sqlx::test(migrations = false)]
    async fn page_past_the_end_is_empty_with_the_real_total(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        create(&db, input("Only note")).await.unwrap();
        let page = list(&db, 9).await.unwrap();
        assert!(page.items.is_empty());
        assert_eq!((page.total, page.page, page.page_count), (1, 9, 1));
    }

    #[sqlx::test(migrations = false)]
    async fn page_zero_means_the_first_page(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        create(&db, input("Only note")).await.unwrap();
        let page = list(&db, 0).await.unwrap();
        assert_eq!((page.page, page.items.len()), (1, 1));
    }

    #[sqlx::test(migrations = false)]
    async fn create_saves_clean_input(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        let note = create(&db, input("  Groceries ")).await.unwrap();
        assert_eq!(get(&db, note.id).await.unwrap().title, "Groceries");
    }

    #[sqlx::test(migrations = false)]
    async fn invalid_input_saves_nothing(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        let error = create(&db, input(" ")).await.unwrap_err();
        assert!(matches!(error, ApiError::Invalid(_)));
        assert_eq!(list(&db, 1).await.unwrap().total, 0);
    }

    #[sqlx::test(migrations = false)]
    async fn missing_note_is_not_found(pool: sqlx::PgPool) {
        let db = test_db(pool).await;
        assert_eq!(get(&db, 404).await.unwrap_err(), ApiError::NotFound);
    }
}
