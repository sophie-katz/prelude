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

use iden::assert_audit_table_iden_valid;
use sea_orm_migration::{prelude::SchemaManager, DbErr};
use sea_query::{Iden, IntoTableRef, TableCreateStatement};
use strum::IntoEnumIterator;

pub mod iden;
pub mod table;

use table::{add_audit_columns, create_audit_trigger, create_table_from_builder, TableKind};

/// Helper function to create an audited table.
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
/// * `table_iden_source` - The identifier of the source table.
/// * `table_iden_audit` - The identifier of the audit table.
/// * `table_builder` - A function that builds the columns of a table.
///
/// # Errors
///
/// Returns any database errors.
///
/// # Panics
///
/// Panics if the audit table identifier enum does not contain the same columns
/// as the source table, plus any standard audit-specific columns.
pub async fn create_audited_table<
    'schema_manager,
    TableIdenSource: IntoTableRef + Iden + IntoEnumIterator + PartialEq + Clone + 'static,
    TableIdenAudit: IntoTableRef + Iden + IntoEnumIterator + PartialEq + Clone + 'static,
    TableBuilder: Fn(TableKind, &mut TableCreateStatement),
>(
    manager: &SchemaManager<'schema_manager>,
    table_iden_source: TableIdenSource,
    table_iden_audit: TableIdenAudit,
    table_builder: &TableBuilder,
) -> Result<(), DbErr> {
    assert_audit_table_iden_valid(table_iden_source.clone(), table_iden_audit.clone());

    create_table_from_builder(
        manager,
        table_iden_source.clone(),
        table_builder,
        TableKind::Source,
    )
    .await?;

    create_table_from_builder(
        manager,
        table_iden_audit.clone(),
        &|table_kind, table_create_statement| {
            table_builder(table_kind, table_create_statement);
            add_audit_columns::<TableIdenAudit>(table_create_statement);
        },
        TableKind::Audit,
    )
    .await?;

    create_audit_trigger(manager, table_iden_source, table_iden_audit).await?;

    Ok(())
}
