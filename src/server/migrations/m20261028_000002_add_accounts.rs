use sea_orm_migration::{async_trait, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(users()).await?;
        manager.alter_table(notes_ownership()).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-notes-owner-id")
                    .table(Notes::Table)
                    .col(Notes::OwnerId)
                    .col(Notes::Id)
                    .to_owned(),
            )
            .await?;
        manager.create_table(audit_events()).await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-audit-events-entity")
                    .table(AuditEvents::Table)
                    .col(AuditEvents::Entity)
                    .col(AuditEvents::EntityId)
                    .to_owned(),
            )
            .await
    }
}

fn users() -> TableCreateStatement {
    Table::create()
        .table(Users::Table)
        .col(big_pk_auto(Users::Id))
        .col(text_uniq(Users::Email))
        .col(text(Users::PasswordHash))
        .col(timestamp_with_time_zone(Users::CreatedAt).default(Expr::current_timestamp()))
        .to_owned()
}

fn notes_ownership() -> TableAlterStatement {
    Table::alter()
        .table(Notes::Table)
        .add_column(big_integer(Notes::OwnerId))
        .add_column(timestamp_with_time_zone_null(Notes::DeletedAt))
        .add_foreign_key(
            TableForeignKey::new()
                .name("fk-notes-owner-id")
                .from_tbl(Notes::Table)
                .from_col(Notes::OwnerId)
                .to_tbl(Users::Table)
                .to_col(Users::Id),
        )
        .to_owned()
}

fn audit_events() -> TableCreateStatement {
    Table::create()
        .table(AuditEvents::Table)
        .col(big_pk_auto(AuditEvents::Id))
        .col(big_integer(AuditEvents::ActorId))
        .col(text(AuditEvents::Action))
        .col(text(AuditEvents::Entity))
        .col(big_integer(AuditEvents::EntityId))
        .col(timestamp_with_time_zone(AuditEvents::CreatedAt).default(Expr::current_timestamp()))
        .foreign_key(
            ForeignKey::create()
                .name("fk-audit-events-actor-id")
                .from(AuditEvents::Table, AuditEvents::ActorId)
                .to(Users::Table, Users::Id),
        )
        .to_owned()
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    PasswordHash,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Notes {
    Table,
    Id,
    OwnerId,
    DeletedAt,
}

#[derive(DeriveIden)]
enum AuditEvents {
    Table,
    Id,
    ActorId,
    Action,
    Entity,
    EntityId,
    CreatedAt,
}
