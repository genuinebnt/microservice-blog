use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notification::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notification::UserId).uuid().not_null())
                    .col(ColumnDef::new(Notification::Kind).string().not_null())
                    .col(ColumnDef::new(Notification::Title).string().not_null())
                    .col(ColumnDef::new(Notification::Message).text().not_null())
                    .col(
                        ColumnDef::new(Notification::IsRead)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Notification::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notification::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Notification {
    #[sea_orm(iden = "notifications")]
    Table,
    Id,
    UserId,
    Kind,
    Title,
    Message,
    IsRead,
    CreatedAt,
}
