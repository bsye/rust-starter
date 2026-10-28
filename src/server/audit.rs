use sea_orm::{ActiveValue::Set, ConnectionTrait, DbErr, EntityTrait};

use super::{auth::Actor, entities::audit_event};

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Created,
    Updated,
    Deleted,
}

impl Action {
    fn as_str(self) -> &'static str {
        match self {
            Action::Created => "created",
            Action::Updated => "updated",
            Action::Deleted => "deleted",
        }
    }
}

pub async fn record(
    conn: &impl ConnectionTrait,
    actor: Actor,
    action: Action,
    entity: &str,
    entity_ids: impl IntoIterator<Item = i64>,
) -> Result<(), DbErr> {
    let events = entity_ids
        .into_iter()
        .map(|entity_id| audit_event::ActiveModel {
            actor_id: Set(actor.id()),
            action: Set(action.as_str().to_string()),
            entity: Set(entity.to_string()),
            entity_id: Set(entity_id),
            ..Default::default()
        });
    audit_event::Entity::insert_many(events)
        .exec_without_returning(conn)
        .await?;
    Ok(())
}
