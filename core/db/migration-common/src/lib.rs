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

//! This module contains any common code for the database migrations.

#![feature(iter_intersperse)]

use sea_orm_migration::{prelude::SchemaManager, sea_orm::ConnectionTrait, DbErr};
use sea_query::{ColumnDef, Iden, IntoTableRef, Table, TableCreateStatement};
use std::collections::HashSet;
use strum::IntoEnumIterator;

/// The standard audit table columns.
static AUDIT_TABLE_COLUMNS: [&str; 8] = [
    "audit_id",
    "audit_action",
    "audit_timestamp_transaction_start",
    "audit_timestamp_statement_start",
    "audit_timestamp_trigger",
    "audit_client_host",
    "audit_client_port",
    "audit_client_query",
];

/// Asserts that the audit table identify enum contains the same columns as the
/// source table, plus any standard audit-specific columns.
///
/// # Type arguments
///
/// * `SourceTableEnum` - The enum that contains the source table identifiers.
/// * `AuditTableEnum` - The enum that contains the audit table identifiers.
///
/// # Examples
///
/// ```
/// use migration_common::assert_audit_table_enum_valid;
/// use sea_query::Iden;
/// use strum_macros::EnumIter;
///
/// #[derive(Iden, EnumIter)]
/// enum ConfigurationTypeReference {
///     Table,
///     Id,
///     Name,
///     Description,
///     DeactivateTimestamp,
/// }

/// #[derive(Iden, EnumIter)]
/// enum ConfigurationTypeReferenceAudit {
///     Table,
///     AuditId,
///     Id,
///     Name,
///     Description,
///     DeactivateTimestamp,
///     AuditAction,
///     AuditTimestampTransactionStart,
///     AuditTimestampStatementStart,
///     AuditTimestampTrigger,
///     AuditClientHost,
///     AuditClientPort,
///     AuditClientQuery,
/// }
///
/// assert_audit_table_enum_valid::<ConfigurationTypeReference, ConfigurationTypeReferenceAudit>();
/// ```
pub fn assert_audit_table_enum_valid<
    SourceTableEnum: IntoEnumIterator + Iden,
    AuditTableEnum: IntoEnumIterator + Iden,
>(
    table_name_source: &str,
    table_name_audit: &str,
) {
    // Collect set of audit table identifiers
    let audit_table_iden_set = AuditTableEnum::iter()
        .map(|x| -> String {
            let mut result = String::new();
            x.unquoted(&mut result);
            result
        })
        .filter(|x| x != table_name_audit)
        .collect::<HashSet<String>>();

    // Assert that the audit table contains all the source table identifiers
    for source_table_iden in SourceTableEnum::iter()
        .map(|x| -> String {
            let mut result = String::new();
            x.unquoted(&mut result);
            result
        })
        .filter(|x| x != table_name_source)
    {
        assert!(audit_table_iden_set.contains(&source_table_iden));
    }

    // Assert that the audit table contains all the standard audit table
    // identifiers
    for audit_table_iden in AUDIT_TABLE_COLUMNS {
        assert!(audit_table_iden_set.contains(audit_table_iden));
    }

    // Assert that the audit table contains no extra identifiers
    //
    // NOTE: Subtract one to account for the filtering out of the table name
    assert_eq!(
        audit_table_iden_set.len(),
        SourceTableEnum::iter().count() - 1 + AUDIT_TABLE_COLUMNS.len()
    );
}

/// An enum to identify whether a table is an audit or source table.
pub enum TableKind {
    /// Identifies a source table
    Source,
    /// Identifies an audit table
    Audit,
}

/// Helper function to create an audited table.
///
/// # Arguments
///
/// * `manager` - A schema manager referenced from the SeaORM migration.
/// * `columns` - An array of column definitions to be added to the table, and
///               also to be audited.
///
/// # Errors
///
/// Returns any database errors.
pub async fn create_audited_table<
    'schema_manager,
    TableIdenSource: IntoTableRef + Iden + IntoEnumIterator + Clone,
    TableIdenAudit: IntoTableRef + Iden + IntoEnumIterator + Clone + 'static,
    TableBuilder: Fn(TableKind, &mut TableCreateStatement),
>(
    manager: &SchemaManager<'schema_manager>,
    table_iden_source: TableIdenSource,
    table_iden_audit: TableIdenAudit,
    table_builder: &TableBuilder,
) -> Result<(), DbErr> {
    create_table_helper(
        manager,
        table_iden_source.clone(),
        table_builder,
        TableKind::Source,
    )
    .await?;

    create_table_helper(
        manager,
        table_iden_audit.clone(),
        &|table_kind, table_create_statement| {
            table_builder(table_kind, table_create_statement);
            add_audit_columns_to_table::<TableIdenAudit>(table_create_statement);
        },
        TableKind::Audit,
    )
    .await?;

    create_audit_trigger(manager, table_iden_source, table_iden_audit).await?;

    Ok(())
}

async fn create_audit_trigger<
    'schema_manager,
    TableIdenSource: Iden + IntoEnumIterator,
    TableIdenAudit: Iden,
>(
    manager: &SchemaManager<'schema_manager>,
    table_iden_source: TableIdenSource,
    table_iden_audit: TableIdenAudit,
) -> Result<(), DbErr> {
    let mut table_name_source = String::new();
    table_iden_source.unquoted(&mut table_name_source);

    let mut table_name_audit = String::new();
    table_iden_audit.unquoted(&mut table_name_audit);

    let column_names = TableIdenSource::iter()
        .map(|x| {
            let mut result = String::new();
            x.unquoted(&mut result);
            result
        })
        .filter(|x| *x != table_name_source)
        .intersperse(", ".to_owned())
        .collect::<String>();

    let column_values = TableIdenSource::iter()
        .map(|x| {
            let mut result = String::new();
            x.unquoted(&mut result);
            result
        })
        .filter(|x| *x != table_name_source)
        .map(|x| format!("CASE TG_OP WHEN 'INSERT' THEN NEW.{x} ELSE OLD.{x} END"))
        .intersperse(", ".to_owned())
        .collect::<String>();

    manager
        .get_connection()
        .execute_unprepared(
            format!(
                r#"
                    CREATE OR REPLACE FUNCTION function_audit_trigger_{table_name_source}()
                    RETURNS TRIGGER AS $body$
                        BEGIN
                            INSERT INTO {table_name_audit} (
                                {column_names},
                                audit_action,
                                audit_timestamp_transaction_start,
                                audit_timestamp_statement_start,
                                audit_timestamp_trigger,
                                audit_client_host,
                                audit_client_port,
                                audit_client_query
                            )
                            VALUES (
                                {column_values},
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

                    CREATE TRIGGER trigger_audit_{table_name_source}
                    BEFORE INSERT OR UPDATE OR DELETE ON {table_name_source}
                    FOR EACH ROW EXECUTE PROCEDURE function_audit_trigger_{table_name_source}();
                "#,
            )
            .as_str(),
        )
        .await
        .map(|_| ())
}

async fn create_table_helper<
    'schema_manager,
    TableIden: IntoTableRef,
    TableBuilder: Fn(TableKind, &mut TableCreateStatement),
>(
    manager: &SchemaManager<'schema_manager>,
    table_iden: TableIden,
    table_builder: &TableBuilder,
    table_kind: TableKind,
) -> Result<(), DbErr> {
    let mut source_table = Table::create();

    source_table.table(table_iden).if_not_exists();

    table_builder(table_kind, &mut source_table);

    manager.create_table(source_table).await
}

fn add_audit_columns_to_table<TableEnum: IntoEnumIterator + Iden + 'static>(
    table_create_statement: &mut TableCreateStatement,
) {
    table_create_statement
        .col(
            ColumnDef::new(
                find_column_with_name::<TableEnum>("audit_id")
                    .expect("required column 'audit_id' missing from audit table"),
            )
            .integer()
            .not_null()
            .auto_increment()
            .primary_key(),
        )
        .col(
            ColumnDef::new(
                find_column_with_name::<TableEnum>("audit_action")
                    .expect("required column 'audit_action' missing from audit table"),
            )
            .char()
            .not_null(),
        )
        .col(
            ColumnDef::new(
                find_column_with_name::<TableEnum>("audit_timestamp_transaction_start").expect(
                    "required column 'audit_timestamp_transaction_start' missing from audit table",
                ),
            )
            .timestamp()
            .not_null(),
        )
        .col(
            ColumnDef::new(
                find_column_with_name::<TableEnum>("audit_timestamp_statement_start").expect(
                    "required column 'audit_timestamp_statement_start' missing from audit table",
                ),
            )
            .timestamp()
            .not_null(),
        )
        .col(
            ColumnDef::new(
                find_column_with_name::<TableEnum>("audit_timestamp_trigger")
                    .expect("required column 'audit_timestamp_trigger' missing from audit table"),
            )
            .timestamp()
            .not_null(),
        )
        .col(
            ColumnDef::new(
                find_column_with_name::<TableEnum>("audit_client_host")
                    .expect("required column 'audit_client_host' missing from audit table"),
            )
            .string()
            .not_null(),
        )
        .col(
            ColumnDef::new(
                find_column_with_name::<TableEnum>("audit_client_port")
                    .expect("required column 'audit_client_port' missing from audit table"),
            )
            .integer()
            .not_null(),
        )
        .col(
            ColumnDef::new(
                find_column_with_name::<TableEnum>("audit_client_query")
                    .expect("required column 'audit_client_query' missing from audit table"),
            )
            .string()
            .not_null(),
        );
}

fn find_column_with_name<TableEnum: IntoEnumIterator + Iden>(name: &str) -> Option<TableEnum> {
    TableEnum::iter().find(|x| -> bool {
        let mut result = String::new();
        x.unquoted(&mut result);
        result == name
    })
}
