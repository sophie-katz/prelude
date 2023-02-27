// MIT License
//
// Copyright (c) 2023 Sophie Katz
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::m20230218_120923_create_configuration_key_reference_table::ConfigurationKeyReference;
use migration_common::{create_audited_table, table::TableKind};
use sea_orm_migration::prelude::*;
use strum_macros::EnumIter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_audited_table(
            manager,
            ConfigurationEntries::Table,
            ConfigurationEntriesAudit::Table,
            &|table_kind, table_create_statement| {
                table_create_statement
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationEntries::Id).integer(),
                        ) {
                            (TableKind::Source, x) => x.not_null().auto_increment().primary_key(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationEntries::KeyId).integer(),
                        ) {
                            (TableKind::Source, x) => x.not_null(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(ColumnDef::new(ConfigurationEntries::UserId).string())
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationEntries::OrderIndex).integer(),
                        ) {
                            (TableKind::Source, x) => x.not_null(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationEntries::Value).string(),
                        ) {
                            (TableKind::Source, x) => x.not_null(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(ColumnDef::new(ConfigurationEntries::DeactivateTimestamp).timestamp());

                if table_kind == TableKind::Source {
                    table_create_statement.foreign_key(
                        ForeignKey::create()
                            .name("foreign_key_configuration_id")
                            .from(ConfigurationEntries::Table, ConfigurationEntries::KeyId)
                            .to(
                                ConfigurationKeyReference::Table,
                                ConfigurationKeyReference::Id,
                            ),
                    );
                }
            },
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ConfigurationEntries::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ConfigurationEntriesAudit::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden, EnumIter, Clone, PartialEq)]
enum ConfigurationEntries {
    Table,
    Id,
    KeyId,
    UserId,
    OrderIndex,
    Value,
    DeactivateTimestamp,
}

#[derive(Iden, EnumIter, Clone, PartialEq)]
enum ConfigurationEntriesAudit {
    Table,
    Id,
    KeyId,
    UserId,
    OrderIndex,
    Value,
    DeactivateTimestamp,
    AuditId,
    AuditAction,
    AuditTimestampTransactionStart,
    AuditTimestampStatementStart,
    AuditTimestampTrigger,
    AuditClientHost,
    AuditClientPort,
    AuditClientQuery,
}
