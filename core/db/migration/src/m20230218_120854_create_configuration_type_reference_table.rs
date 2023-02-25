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

use sea_orm_migration::{prelude::*, sea_orm::ConnectionTrait};
use strum_macros::EnumIter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConfigurationTypeReference::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConfigurationTypeReference::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationTypeReference::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationTypeReference::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationTypeReference::DeactivateTimestamp).timestamp(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ConfigurationTypeReferenceAudit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConfigurationTypeReferenceAudit::AuditId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ConfigurationTypeReferenceAudit::Id).integer())
                    .col(ColumnDef::new(ConfigurationTypeReferenceAudit::Name).string())
                    .col(ColumnDef::new(ConfigurationTypeReferenceAudit::Description).string())
                    .col(
                        ColumnDef::new(ConfigurationTypeReferenceAudit::DeactivateTimestamp)
                            .timestamp(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationTypeReferenceAudit::AuditAction)
                            .char()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(
                            ConfigurationTypeReferenceAudit::AuditTimestampTransactionStart,
                        )
                        .timestamp()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(
                            ConfigurationTypeReferenceAudit::AuditTimestampStatementStart,
                        )
                        .timestamp()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationTypeReferenceAudit::AuditTimestampTrigger)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationTypeReferenceAudit::AuditClientHost)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationTypeReferenceAudit::AuditClientPort)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfigurationTypeReferenceAudit::AuditClientQuery)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE OR REPLACE FUNCTION function_audit_trigger_configuration_type_reference()
                    RETURNS TRIGGER AS $body$
                        BEGIN
                            INSERT INTO configuration_type_reference_audit (
                                id,
                                name,
                                description,
                                deactivate_timestamp,
                                audit_action,
                                audit_timestamp_transaction_start,
                                audit_timestamp_statement_start,
                                audit_timestamp_trigger,
                                audit_client_host,
                                audit_client_port,
                                audit_client_query
                            )
                            VALUES (
                                CASE TG_OP
                                    WHEN 'INSERT' THEN NEW.id
                                    ELSE OLD.id
                                END,
                                CASE TG_OP
                                    WHEN 'INSERT' THEN NEW.name
                                    ELSE OLD.name
                                END,
                                CASE TG_OP
                                    WHEN 'INSERT' THEN NEW.description
                                    ELSE OLD.description
                                END,
                                CASE TG_OP
                                    WHEN 'INSERT' THEN NEW.deactivate_timestamp
                                    ELSE OLD.deactivate_timestamp
                                END,
                                CASE TG_OP
                                    WHEN 'INSERT' THEN 'I'
                                    WHEN 'UPDATE' THEN 'U'
                                    WHEN 'DELETE' THEN 'D'
                                    ELSE '?'
                                END,
                                current_timestamp,
                                statement_timestamp(),
                                clock_timestamp(),
                                inet_client_addr(),
                                inet_client_port(),
                                current_query()
                            );

                            IF TG_OP = 'DELETE' THEN
                                RETURN OLD;
                            ELSE
                                RETURN NEW;
                            END IF;
                        END
                    $body$ LANGUAGE 'plpgsql';

                    CREATE TRIGGER trigger_audit_configuration_type_reference
                    BEFORE INSERT OR UPDATE OR DELETE ON configuration_type_reference
                    FOR EACH ROW EXECUTE PROCEDURE function_audit_trigger_configuration_type_reference();
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ConfigurationTypeReference::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ConfigurationTypeReferenceAudit::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden, EnumIter)]
pub enum ConfigurationTypeReference {
    Table,
    Id,
    Name,
    Description,
    DeactivateTimestamp,
}

#[derive(Iden, EnumIter)]
pub enum ConfigurationTypeReferenceAudit {
    Table,
    AuditId,
    Id,
    Name,
    Description,
    DeactivateTimestamp,
    AuditAction,
    AuditTimestampTransactionStart,
    AuditTimestampStatementStart,
    AuditTimestampTrigger,
    AuditClientHost,
    AuditClientPort,
    AuditClientQuery,
}

#[cfg(test)]
mod tests {
    use super::{ConfigurationTypeReference, ConfigurationTypeReferenceAudit};
    use migration_common::assert_audit_table_enum_valid;

    #[test]
    fn test_audit_table_columns() {
        assert_audit_table_enum_valid::<ConfigurationTypeReference, ConfigurationTypeReferenceAudit>(
        );
    }
}
