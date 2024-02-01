use sea_orm_migration::prelude::*;
// use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Uuid).uuid().unique_key().not_null())
                    .col(ColumnDef::new(User::ProfilePic).binary().null())
                    .col(
                        ColumnDef::new(User::IsAdmin)
                            .boolean()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name("idx-indexes-email")
                            .table(User::Table)
                            .col(User::Email),
                    )
                    .index(
                        Index::create()
                            .name("idx-indexes-id")
                            .table(User::Table)
                            .col(User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    Uuid,
    ProfilePic,
    CreatedAt,
    UpdatedAt,
    IsAdmin,
}
