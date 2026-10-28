use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "audit_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub actor_id: i64,
    pub action: String,
    pub entity: String,
    pub entity_id: i64,
    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
