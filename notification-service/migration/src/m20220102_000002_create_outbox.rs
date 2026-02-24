use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Outbox::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Outbox::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Outbox::AggregateType).string().not_null())
                    .col(ColumnDef::new(Outbox::AggregateId).uuid().not_null())
                    .col(ColumnDef::new(Outbox::EventType).string().not_null())
                    .col(ColumnDef::new(Outbox::Payload).json_binary().not_null())
                    .col(
                        ColumnDef::new(Outbox::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Outbox::SentAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_outbox_unsent")
                    .table(Outbox::Table)
                    .col(Outbox::SentAt)
                    .col(Outbox::CreatedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Outbox::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Outbox {
    #[sea_orm(iden = "outbox")]
    Table,
    Id,
    AggregateType,
    AggregateId,
    EventType,
    Payload,
    CreatedAt,
    SentAt,
}
