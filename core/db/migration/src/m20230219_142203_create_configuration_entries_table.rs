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
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConfigurationEntries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConfigurationEntries::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationEntries::KeyId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ConfigurationEntries::UserId).string())
                    .col(
                        ColumnDef::new(ConfigurationEntries::Order)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationEntries::Value)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationEntries::CreateTimestamp)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ConfigurationEntries::DeactivateTimestamp).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("foreign_key_configuration_id")
                            .from(ConfigurationEntries::Table, ConfigurationEntries::KeyId)
                            .to(
                                ConfigurationKeyReference::Table,
                                ConfigurationKeyReference::Id,
                            ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ConfigurationEntries::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ConfigurationEntries {
    Table,
    Id,
    KeyId,
    UserId,
    Order,
    Value,
    CreateTimestamp,
    DeactivateTimestamp,
}
