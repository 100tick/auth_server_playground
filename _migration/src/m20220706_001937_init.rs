use entity::gender;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220706_002224_init"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TempUser::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TempUser::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(TempUser::GoogleId)
                            .char_len(21)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(TempUser::Email)
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(TempUser::AvatarUrl).string().not_null())
                    .col(ColumnDef::new(TempUser::Name).string_len(20).not_null())
                    .col(
                        ColumnDef::new(TempUser::FirstName)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(ColumnDef::new(TempUser::LastName).string_len(20).not_null())
                    .col(ColumnDef::new(TempUser::Locale).char_len(2).not_null())
                    .col(ColumnDef::new(TempUser::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(TempUser::ExpiresAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(User::GoogleId)
                            .char_len(21)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string_len(320)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::AvatarUrl).string().not_null())
                    .col(ColumnDef::new(User::Name).string_len(50).not_null())
                    .col(ColumnDef::new(User::Locale).char_len(2).not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserInfo::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(UserInfo::UserId)
                            .char_len(21)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserInfo::FirstName)
                            .string_len(30)
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserInfo::LastName).string_len(30).not_null())
                    // .col(ColumnDef::new(UserInfo::Gender).string().not_null())
                    .col(ColumnDef::new(UserInfo::Gender).string_len(7).not_null())
                    .col(ColumnDef::new(UserInfo::BirthDate).date().not_null())
                    .col(
                        ColumnDef::new(UserInfo::Phone)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserInfo::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(UserInfo::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TempUser::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserInfo::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum TempUser {
    Table,
    Id,
    GoogleId,
    AvatarUrl,
    Email,
    Name,
    FirstName,
    LastName,
    Gender,
    Locale,
    CreatedAt,
    ExpiresAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    GoogleId,
    AvatarUrl,
    Email,
    Name,
    Locale,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum UserInfo {
    Table,
    Id,
    UserId,
    FirstName,
    LastName,
    Gender,
    BirthDate,
    Phone,
    CreatedAt,
    UpdatedAt,
}
