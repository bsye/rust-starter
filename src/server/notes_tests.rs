use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use super::*;
use crate::server::{auth::test_actor, db::test_db, entities::audit_event};

fn input(title: &str) -> NoteInput {
    NoteInput {
        title: title.to_string(),
        body: String::new(),
    }
}

async fn two_users(db: &DatabaseConnection) -> (Actor, Actor) {
    (
        test_actor(db, "ada@example.com").await,
        test_actor(db, "bob@example.com").await,
    )
}

#[test]
fn every_note_read_goes_through_scoped() {
    let source = include_str!("notes.rs");
    let unscoped_find = ["note::Entity", "::find()"].concat();
    assert_eq!(
        source.matches(unscoped_find.as_str()).count(),
        1,
        "only scoped() may call find()"
    );
}

#[sqlx::test(migrations = false)]
async fn lists_newest_first_with_totals(pool: sqlx::PgPool) {
    let db = test_db(pool).await;
    let (ada, _) = two_users(&db).await;
    for n in 1..=NOTES_PAGE_SIZE + 1 {
        create(&db, ada, input(&format!("Note {n}"))).await.unwrap();
    }
    let first = list(&db, ada, 1).await.unwrap();
    assert_eq!(first.items.len() as u64, NOTES_PAGE_SIZE);
    assert_eq!(
        first.items[0].title,
        format!("Note {}", NOTES_PAGE_SIZE + 1)
    );
    assert_eq!((first.total, first.page_count), (NOTES_PAGE_SIZE + 1, 2));
    assert_eq!(list(&db, ada, 2).await.unwrap().items.len(), 1);
}

#[sqlx::test(migrations = false)]
async fn page_past_the_end_is_empty_with_the_real_total(pool: sqlx::PgPool) {
    let db = test_db(pool).await;
    let (ada, _) = two_users(&db).await;
    create(&db, ada, input("Only note")).await.unwrap();
    let page = list(&db, ada, 9).await.unwrap();
    assert!(page.items.is_empty());
    assert_eq!((page.total, page.page, page.page_count), (1, 9, 1));
    assert_eq!(list(&db, ada, 0).await.unwrap().page, 1);
}

#[sqlx::test(migrations = false)]
async fn nobody_sees_or_changes_another_users_notes(pool: sqlx::PgPool) {
    let db = test_db(pool).await;
    let (ada, bob) = two_users(&db).await;
    let note = create(&db, ada, input("Ada's note")).await.unwrap();
    assert_eq!(list(&db, bob, 1).await.unwrap().total, 0);
    assert_eq!(get(&db, bob, note.id).await, Err(ApiError::NotFound));
    assert_eq!(
        update(&db, bob, note.id, input("Hijacked")).await,
        Err(ApiError::NotFound)
    );
    assert_eq!(delete(&db, bob, note.id).await, Err(ApiError::NotFound));
    assert_eq!(get(&db, ada, note.id).await.unwrap().title, "Ada's note");
}

#[sqlx::test(migrations = false)]
async fn invalid_input_saves_nothing(pool: sqlx::PgPool) {
    let db = test_db(pool).await;
    let (ada, _) = two_users(&db).await;
    assert!(matches!(
        create(&db, ada, input(" ")).await,
        Err(ApiError::Invalid(_))
    ));
    assert_eq!(list(&db, ada, 1).await.unwrap().total, 0);
}

#[sqlx::test(migrations = false)]
async fn update_saves_clean_input_and_moves_updated_at(pool: sqlx::PgPool) {
    let db = test_db(pool).await;
    let (ada, _) = two_users(&db).await;
    let note = create(&db, ada, input("Draft")).await.unwrap();
    let updated = update(&db, ada, note.id, input("  Final ")).await.unwrap();
    assert_eq!(updated.title, "Final");
    assert!(updated.updated_at > note.updated_at);
}

#[sqlx::test(migrations = false)]
async fn delete_hides_the_note_but_keeps_the_row(pool: sqlx::PgPool) {
    let db = test_db(pool).await;
    let (ada, _) = two_users(&db).await;
    let note = create(&db, ada, input("Old")).await.unwrap();
    delete(&db, ada, note.id).await.unwrap();
    assert_eq!(get(&db, ada, note.id).await, Err(ApiError::NotFound));
    assert_eq!(list(&db, ada, 1).await.unwrap().total, 0);
    let row = note::Entity::find_by_id(note.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert!(row.deleted_at.is_some());
}

#[sqlx::test(migrations = false)]
async fn every_change_records_who_and_what(pool: sqlx::PgPool) {
    let db = test_db(pool).await;
    let (ada, _) = two_users(&db).await;
    let note = create(&db, ada, input("Draft")).await.unwrap();
    update(&db, ada, note.id, input("Final")).await.unwrap();
    delete(&db, ada, note.id).await.unwrap();
    let events = audit_event::Entity::find()
        .filter(audit_event::Column::EntityId.eq(note.id))
        .all(&db)
        .await
        .unwrap();
    let actions: Vec<_> = events.iter().map(|event| event.action.as_str()).collect();
    assert_eq!(actions, ["created", "updated", "deleted"]);
    assert!(
        events
            .iter()
            .all(|event| event.actor_id == ada.id() && event.entity == "note")
    );
}

#[sqlx::test(migrations = false)]
async fn insert_many_audits_every_note(pool: sqlx::PgPool) {
    let db = test_db(pool).await;
    let (ada, _) = two_users(&db).await;
    insert_many(&db, ada, vec![input("One"), input("Two")])
        .await
        .unwrap();
    assert_eq!(list(&db, ada, 1).await.unwrap().total, 2);
    assert_eq!(audit_event::Entity::find().all(&db).await.unwrap().len(), 2);
}
