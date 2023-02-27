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

use super::m20230218_120854_create_configuration_type_reference_table::ConfigurationTypeReference;
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
            ConfigurationKeyReference::Table,
            ConfigurationKeyReferenceAudit::Table,
            &|table_kind, table_create_statement| {
                table_create_statement
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationKeyReference::Id).integer(),
                        ) {
                            (TableKind::Source, x) => x.not_null().auto_increment().primary_key(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationKeyReference::Name).string(),
                        ) {
                            (TableKind::Source, x) => x.not_null().unique_key(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationKeyReference::Description).string(),
                        ) {
                            (TableKind::Source, x) => x.not_null(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationKeyReference::TypeId).integer(),
                        ) {
                            (TableKind::Source, x) => x.not_null(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationKeyReference::Optional).boolean(),
                        ) {
                            (TableKind::Source, x) => x.not_null(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationKeyReference::AllowsMultiple).boolean(),
                        ) {
                            (TableKind::Source, x) => x.not_null(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        match (
                            &table_kind,
                            ColumnDef::new(ConfigurationKeyReference::AllowsUserOverride).boolean(),
                        ) {
                            (TableKind::Source, x) => x.not_null(),
                            (TableKind::Audit, x) => x,
                        },
                    )
                    .col(
                        ColumnDef::new(ConfigurationKeyReference::DeactivateTimestamp).timestamp(),
                    );

                if table_kind == TableKind::Source {
                    table_create_statement.foreign_key(
                        ForeignKey::create()
                            .name("foreign_key_configuration_reference_type_id")
                            .from(
                                ConfigurationKeyReference::Table,
                                ConfigurationKeyReference::TypeId,
                            )
                            .to(
                                ConfigurationTypeReference::Table,
                                ConfigurationTypeReference::Id,
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
            .drop_table(
                Table::drop()
                    .table(ConfigurationKeyReference::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ConfigurationKeyReferenceAudit::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden, EnumIter, Clone, PartialEq)]
pub enum ConfigurationKeyReference {
    Table,
    Id,
    Name,
    Description,
    TypeId,
    Optional,
    AllowsMultiple,
    AllowsUserOverride,
    DeactivateTimestamp,
}

#[derive(Iden, EnumIter, Clone, PartialEq)]
pub enum ConfigurationKeyReferenceAudit {
    Table,
    Id,
    Name,
    Description,
    TypeId,
    Optional,
    AllowsMultiple,
    AllowsUserOverride,
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
