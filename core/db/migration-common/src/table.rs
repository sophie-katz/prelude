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

//! Utility functions for creating audit tables.

use crate::iden::{find_column_with_name, get_iden_name, iterate_table_columns};
use sea_orm_migration::{prelude::SchemaManager, sea_orm::ConnectionTrait, DbErr};
use sea_query::{ColumnDef, Iden, IntoTableRef, Table, TableCreateStatement};
use strum::IntoEnumIterator;

/// An enum to identify whether a table is an audit or source table.
#[derive(Debug, PartialEq)]
pub enum TableKind {
    /// Identifies a source table
    Source,
    /// Identifies an audit table
    Audit,
}

/// Creates a table in a database using a table builder function.
///
/// A builder function looks like this:
///
/// ```
/// use sea_query::{Iden, ColumnDef, TableCreateStatement};
/// use strum_macros::EnumIter;
/// use migration_common::table::TableKind;
///
/// #[derive(Iden, EnumIter, Clone)]
/// pub enum ConfigurationTypeReference {
///     Table,
///     Id,
/// }
///
/// fn builder(
///     table_kind: TableKind,
///     table_create_statement: &mut TableCreateStatement
/// ) {
///     table_create_statement
///         .col(
///             match (
///                 &table_kind,
///                 ColumnDef::new(ConfigurationTypeReference::Id).integer(),
///             ) {
///                 (TableKind::Source, x) => x.not_null().auto_increment()
///                     .primary_key(),
///                 (TableKind::Audit, x) => x,
///             },
///         );
/// }
/// ```
///
/// Within the builder function, the `table_create_statement` is built up
/// column-by-column using the `.col()` method. Each column can be defined as
/// above using a `match` statement to change column behavior depending on
/// whether or not the column is being added to a source or audit table.
///
/// * When adding to a source table, the column should be restricted using
///   constraints as needed.
/// * When adding to an audit table, the column should be completely
///   unconstrained to prevent exceptions being caused when inserting into the
///   audit table.
///
/// # Arguments
///
/// * `manager` - A schema manager referenced from the SeaORM migration.
/// * `table_iden` - The table identifier to use to name the table.
/// * `table_builder` - A function that builds the columns of a table.
/// * `table_kind` - An enum to identify whether a table is an audit or source
///                  table.
///
/// # Errors
///
/// Returns any database errors.
pub async fn create_table_from_builder<
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

/// Add audit columns to an existing table.
///
/// This function requires the table to have the following columns:
///
/// * `AuditId` - A primary key to use for audit entries.
/// * `AuditAction` - `'I'` to represent inserts, `'U'` to represent updates,
///                   and `'D'` to represent deletes.
/// * `AuditTimestampTransactionStart` - The timestamp of the start of the
///                                      transaction being audited.
/// * `AuditTimestampStatementStart` - The timestamp of the start of the
///                                    statement being audited.
/// * `AuditTimestampTrigger` - The timestamp of the audit trigger being called.
/// * `AuditClientHost` - The network host of the client that initiated the
///                       transaction.
/// * `AuditClientPort` - The TCP port of the client that initiated the
///                       transaction.
/// * `AuditClientQuery` - The text of the top-level PostgreSQL query using
///                        executed.
///
/// These must all be defined in the table's identifier enum. For example:
///
/// ```
/// use sea_query::Iden;
/// use strum_macros::EnumIter;
///
/// #[derive(Iden, EnumIter, PartialEq)]
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
/// ```
///
/// # Arguments
///
/// * `table_create_statement` - The table to add audit columns to.
///
/// # Panics
///
/// Panics if any of the required audit columns are missing from the table.
pub fn add_audit_columns<TableEnum: IntoEnumIterator + Iden + 'static>(
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

/// Creates an audit trigger for a table.
///
/// # Arguments
///
/// * `manager` - A schema manager referenced from the SeaORM migration.
/// * `table_iden_source` - The identifier of the source table.
/// * `table_iden_audit` - The identifier of the audit table.
///
/// # Errors
///
/// Returns any database errors.
pub async fn create_audit_trigger<
    'schema_manager,
    TableIdenSource: Iden + IntoEnumIterator + PartialEq + Clone + 'static,
    TableIdenAudit: Iden + PartialEq + Clone,
>(
    manager: &SchemaManager<'schema_manager>,
    table_iden_source: TableIdenSource,
    table_iden_audit: TableIdenAudit,
) -> Result<(), DbErr> {
    manager
        .get_connection()
        .execute_unprepared(
            create_audit_trigger_unprepared(table_iden_source, table_iden_audit).as_str(),
        )
        .await
        .map(|_| ())
}

pub(crate) fn create_audit_trigger_unprepared<
    TableIdenSource: Iden + IntoEnumIterator + PartialEq + Clone + 'static,
    TableIdenAudit: Iden + PartialEq + Clone,
>(
    table_iden_source: TableIdenSource,
    table_iden_audit: TableIdenAudit,
) -> String {
    let table_name_source = get_iden_name(&table_iden_source);
    let table_name_audit = get_iden_name(&table_iden_audit);

    let column_names = iterate_table_columns(table_iden_source.clone())
        .map(|x| get_iden_name(&x))
        .intersperse(", ".to_owned())
        .collect::<String>();

    let column_values = iterate_table_columns(table_iden_source)
        .map(|x| get_iden_name(&x))
        .map(|x| format!("CASE TG_OP WHEN 'INSERT' THEN NEW.{x} ELSE OLD.{x} END"))
        .intersperse(", ".to_owned())
        .collect::<String>();

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
}

#[cfg(test)]
mod tests {
    use super::create_audit_trigger_unprepared;
    use sea_query::Iden;
    use strum_macros::EnumIter;

    #[derive(Iden, EnumIter, PartialEq, Clone, Debug)]
    enum SourceOneColumn {
        Table,
        Col0,
    }

    #[derive(Iden, EnumIter, PartialEq, Clone, Debug)]
    enum AuditOneColumn {
        Table,
        Col0,
        AuditId,
        AuditAction,
        AuditTimestampTransactionStart,
        AuditTimestampStatementStart,
        AuditTimestampTrigger,
        AuditClientHost,
        AuditClientPort,
        AuditClientQuery,
    }

    #[derive(Iden, EnumIter, PartialEq, Clone, Debug)]
    enum SourceTwoColumns {
        Table,
        Col0,
        Col1,
    }

    #[derive(Iden, EnumIter, PartialEq, Clone, Debug)]
    enum AuditTwoColumns {
        Table,
        Col0,
        Col1,
        AuditId,
        AuditAction,
        AuditTimestampTransactionStart,
        AuditTimestampStatementStart,
        AuditTimestampTrigger,
        AuditClientHost,
        AuditClientPort,
        AuditClientQuery,
    }

    #[test]
    fn test_create_audit_trigger_unprepared_one_column() {
        let expected = r#"
            CREATE OR REPLACE FUNCTION function_audit_trigger_source_one_column()
            RETURNS TRIGGER AS $body$
                BEGIN
                    INSERT INTO audit_one_column (
                        col0,
                        audit_action,
                        audit_timestamp_transaction_start,
                        audit_timestamp_statement_start,
                        audit_timestamp_trigger,
                        audit_client_host,
                        audit_client_port,
                        audit_client_query
                    )
                    VALUES (
                        CASE TG_OP WHEN 'INSERT' THEN NEW.col0 ELSE OLD.col0 END,
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

            CREATE TRIGGER trigger_audit_source_one_column
            BEFORE INSERT OR UPDATE OR DELETE ON source_one_column
            FOR EACH ROW EXECUTE PROCEDURE function_audit_trigger_source_one_column();
        "#;

        assert_eq!(
            expected,
            create_audit_trigger_unprepared(SourceOneColumn::Table, AuditOneColumn::Table)
        );
    }

    #[test]
    fn test_create_audit_trigger_unprepared_two_columns() {
        let expected = r#"
            CREATE OR REPLACE FUNCTION function_audit_trigger_source_two_columns()
            RETURNS TRIGGER AS $body$
                BEGIN
                    INSERT INTO audit_two_columns (
                        col0, col1,
                        audit_action,
                        audit_timestamp_transaction_start,
                        audit_timestamp_statement_start,
                        audit_timestamp_trigger,
                        audit_client_host,
                        audit_client_port,
                        audit_client_query
                    )
                    VALUES (
                        CASE TG_OP WHEN 'INSERT' THEN NEW.col0 ELSE OLD.col0 END, CASE TG_OP WHEN 'INSERT' THEN NEW.col1 ELSE OLD.col1 END,
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

            CREATE TRIGGER trigger_audit_source_two_columns
            BEFORE INSERT OR UPDATE OR DELETE ON source_two_columns
            FOR EACH ROW EXECUTE PROCEDURE function_audit_trigger_source_two_columns();
        "#;

        assert_eq!(
            expected,
            create_audit_trigger_unprepared(SourceTwoColumns::Table, AuditTwoColumns::Table)
        );
    }
}
