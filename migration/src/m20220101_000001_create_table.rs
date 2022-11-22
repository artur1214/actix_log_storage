use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                .table(Request::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Request::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(Request::User).string().null())
                .col(ColumnDef::new(Request::Data).string().null())
                .col(ColumnDef::new(Request::DocumentId).string().null())
                .col(ColumnDef::new(Request::DocumentLifecycle).string().null())
                .col(ColumnDef::new(Request::FrameId).integer().null())
                .col(ColumnDef::new(Request::FrameType).string().null())
                .col(ColumnDef::new(Request::FromCache).boolean().null())
                .col(ColumnDef::new(Request::Initiator).string().null())
                .col(ColumnDef::new(Request::Ip).string().null())
                .col(ColumnDef::new(Request::Method).string().null())
                .col(ColumnDef::new(Request::ParentDocumentId).string().null())
                .col(ColumnDef::new(Request::ParentFrameId).integer().null())
                .col(ColumnDef::new(Request::RequestId).string().null())
                .col(ColumnDef::new(Request::ResponseHeaders).string().null())
                .col(ColumnDef::new(Request::StatusCode).integer().null())
                .col(ColumnDef::new(Request::StatusLine).string().null())
                .col(ColumnDef::new(Request::TabId).integer().null())
                .col(ColumnDef::new(Request::TimeStamp).timestamp().null())
                .col(ColumnDef::new(Request::Type).string().null())
                .col(ColumnDef::new(Request::Url).string().null())
                .col(ColumnDef::new(Request::ServerTimeStamp).timestamp().null())
                .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Request::Table)
                    .to_owned()
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Request {
    Table,
    Id,
    User,
    Data,
    DocumentId,
    DocumentLifecycle,
    FrameId,
    FrameType,
    FromCache,
    Initiator,
    Ip,
    Method,
    ParentDocumentId,
    ParentFrameId,
    RequestId,
    ResponseHeaders,
    StatusCode,
    StatusLine,
    TabId,
    TimeStamp,
    Type,
    Url,
    ServerTimeStamp
}
