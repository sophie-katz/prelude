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

//! Utility functions for dealing with SeaQuery identifier enums.

use sea_query::Iden;
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

/// Asserts that the audit table identifier enum contains the same columns as
/// the source table, plus any standard audit-specific columns.
///
/// # Type arguments
///
/// * `SourceTableIDen` - The enum that contains the source table identifiers.
/// * `AuditTableIDen` - The enum that contains the audit table identifiers.
///
/// # Examples
///
/// ```
/// use migration_common::iden::assert_audit_table_iden_valid;
/// use sea_query::Iden;
/// use strum_macros::EnumIter;
///
/// #[derive(Iden, EnumIter, PartialEq)]
/// enum ConfigurationTypeReference {
///     Table,
///     Id,
///     Name,
///     Description,
///     DeactivateTimestamp,
/// }

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
///
/// assert_audit_table_iden_valid(
///     ConfigurationTypeReference::Table,
///     ConfigurationTypeReferenceAudit::Table
/// );
/// ```
///
/// # Panics
///
/// Panics if the audit table identifier enum does not contain the same columns
/// as the source table, plus any standard audit-specific columns.
pub fn assert_audit_table_iden_valid<
    TableIdenSource: IntoEnumIterator + Iden + PartialEq + 'static,
    TableIdenAudit: IntoEnumIterator + Iden + PartialEq + 'static,
>(
    table_iden_source: TableIdenSource,
    table_iden_audit: TableIdenAudit,
) {
    // Collect set of audit table identifiers
    let audit_table_iden_set = iterate_table_columns(table_iden_audit)
        .map(|x| get_iden_name(&x))
        .collect::<HashSet<String>>();

    // Assert that the audit table contains all the source table identifiers
    for table_iden_source_entry in iterate_table_columns(table_iden_source) {
        assert!(audit_table_iden_set.contains(&get_iden_name(&table_iden_source_entry)));
    }

    // Assert that the audit table contains all the standard audit table
    // identifiers
    for audit_table_iden in AUDIT_TABLE_COLUMNS {
        assert!(audit_table_iden_set.contains(audit_table_iden));
    }

    // Assert that the audit table contains no extra identifiers
    assert_eq!(
        audit_table_iden_set.len(),
        count_table_columns::<TableIdenSource>() + AUDIT_TABLE_COLUMNS.len()
    );
}

/// Iterates over the column identifiers for a given table.
///
/// # Arguments
///
/// * `table_iden` - The table name identifier for the given table.
///
/// # Examples
///
/// ```
/// use migration_common::iden::iterate_table_columns;
/// use sea_query::Iden;
/// use strum_macros::EnumIter;
///
/// #[derive(Iden, EnumIter, PartialEq, Debug)]
/// enum ConfigurationTypeReference {
///     Table,
///     Id,
///     Name,
///     Description,
///     DeactivateTimestamp,
/// }
///
/// let columns = iterate_table_columns(ConfigurationTypeReference::Table)
///     .collect::<Vec<ConfigurationTypeReference>>();
///
/// assert_eq!(
///     columns,
///     vec![
///         ConfigurationTypeReference::Id,
///         ConfigurationTypeReference::Name,
///         ConfigurationTypeReference::Description,
///         ConfigurationTypeReference::DeactivateTimestamp
///     ]
/// );
/// ```
pub fn iterate_table_columns<TableIden: IntoEnumIterator + PartialEq + 'static>(
    table_iden: TableIden,
) -> impl Iterator<Item = TableIden> {
    TableIden::iter().filter(move |x| *x != table_iden)
}

/// Gets the name of the given identifier.
///
/// # Arguments
///
/// * `table_iden` - The identifier.
///
/// # Examples
///
/// ```
/// use migration_common::iden::get_iden_name;
/// use sea_query::Iden;
/// use strum_macros::EnumIter;
///
/// #[derive(Iden, EnumIter, PartialEq, Debug)]
/// enum ConfigurationTypeReference {
///     Table,
///     Id,
///     Name,
///     Description,
///     DeactivateTimestamp,
/// }
///
/// assert_eq!(
///     get_iden_name(&ConfigurationTypeReference::Table),
///     "configuration_type_reference"
/// );
///
/// assert_eq!(
///     get_iden_name(&ConfigurationTypeReference::Id),
///     "id"
/// );
///
/// assert_eq!(
///     get_iden_name(&ConfigurationTypeReference::DeactivateTimestamp),
///     "deactivate_timestamp"
/// );
/// ```
pub fn get_iden_name<TableIden: Iden>(table_iden: &TableIden) -> String {
    let mut result = String::new();
    table_iden.unquoted(&mut result);

    assert!(!result.is_empty(), "table identifier must not be empty");

    result
}

/// Counts the number of columns in the given table.
///
/// # Type arguments
///
/// * `TableIden` - The type of the table identifier enum.
///
/// # Examples
///
/// ```
/// use migration_common::iden::count_table_columns;
/// use sea_query::Iden;
/// use strum_macros::EnumIter;
///
/// #[derive(Iden, EnumIter, PartialEq, Debug)]
/// enum ConfigurationTypeReference {
///     Table,
///     Id,
///     Name,
///     Description,
///     DeactivateTimestamp,
/// }
///
/// assert_eq!(count_table_columns::<ConfigurationTypeReference>(), 4);
/// ```
pub fn count_table_columns<TableIden: IntoEnumIterator>() -> usize {
    let count = TableIden::iter().count();

    assert!(
        count > 0,
        "table identifier enum must contain at least one entry"
    );

    count - 1
}

/// Finds the identifier of a column with a given name.
///
/// # Type arguments
///
/// * `TableIden` - The type of the table identifier enum.
///
/// # Arguments
///
/// * `name` - The name of the column to find.
///
/// # Returns
///
/// Returns either the identifier of the column with the given name or `None` if
/// no column with `name` exists.
///
/// # Examples
///
/// ```
/// use migration_common::iden::find_column_with_name;
/// use sea_query::Iden;
/// use strum_macros::EnumIter;
///
/// #[derive(Iden, EnumIter, PartialEq, Debug)]
/// enum ConfigurationTypeReference {
///     Table,
///     Id,
///     Name,
///     Description,
///     DeactivateTimestamp,
/// }
///
/// assert_eq!(
///     find_column_with_name::<ConfigurationTypeReference>("id"),
///     Some(ConfigurationTypeReference::Id)
/// );
///
/// assert_eq!(
///     find_column_with_name::<ConfigurationTypeReference>(
///         "deactivate_timestamp"
///     ),
///     Some(ConfigurationTypeReference::DeactivateTimestamp)
/// );
///
/// assert_eq!(
///     find_column_with_name::<ConfigurationTypeReference>(
///         "configuration_type_reference"
///     ),
///     Some(ConfigurationTypeReference::Table)
/// );
/// ```
pub fn find_column_with_name<TableIden: IntoEnumIterator + Iden>(name: &str) -> Option<TableIden> {
    TableIden::iter().find(|x| -> bool {
        let mut result = String::new();
        x.unquoted(&mut result);
        result == name
    })
}

#[cfg(test)]
mod tests {
    use sea_query::Iden;
    use strum_macros::EnumIter;

    use super::{
        assert_audit_table_iden_valid, count_table_columns, find_column_with_name, get_iden_name,
        iterate_table_columns,
    };

    #[derive(Iden, EnumIter, PartialEq, Debug)]
    enum TableZeroColumns {
        Table,
    }

    #[derive(Iden, EnumIter, PartialEq, Debug)]
    enum SourceOneColumn {
        Table,
        Col0,
    }

    #[derive(Iden, EnumIter, PartialEq, Debug)]
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

    #[derive(Iden, EnumIter, PartialEq, Debug)]
    enum AuditOneColumnMissingAuditColumn {
        Table,
        Col0,
        AuditAction,
        AuditTimestampTransactionStart,
        AuditTimestampStatementStart,
        AuditTimestampTrigger,
        AuditClientHost,
        AuditClientPort,
        AuditClientQuery,
    }

    #[derive(Iden, EnumIter, PartialEq, Debug)]
    enum AuditOneColumnMissingSourceColumn {
        Table,
        AuditId,
        AuditAction,
        AuditTimestampTransactionStart,
        AuditTimestampStatementStart,
        AuditTimestampTrigger,
        AuditClientHost,
        AuditClientPort,
        AuditClientQuery,
    }

    #[derive(Iden, EnumIter, PartialEq, Debug)]
    enum SourceTwoColumns {
        Table,
        Col0,
        Col1,
    }

    #[derive(Iden, EnumIter, PartialEq, Debug)]
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
    fn test_iterate_table_columns_zero_columns() {
        let columns =
            iterate_table_columns(TableZeroColumns::Table).collect::<Vec<TableZeroColumns>>();

        assert_eq!(columns, vec![])
    }

    #[test]
    fn test_iterate_table_columns_one_column() {
        let columns =
            iterate_table_columns(SourceOneColumn::Table).collect::<Vec<SourceOneColumn>>();

        assert_eq!(columns, vec![SourceOneColumn::Col0])
    }

    #[test]
    fn test_iterate_table_columns_two_columns() {
        let columns =
            iterate_table_columns(SourceTwoColumns::Table).collect::<Vec<SourceTwoColumns>>();

        assert_eq!(
            columns,
            vec![SourceTwoColumns::Col0, SourceTwoColumns::Col1]
        )
    }

    #[test]
    fn test_get_iden_name() {
        assert_eq!(get_iden_name(&SourceOneColumn::Table), "source_one_column");
        assert_eq!(get_iden_name(&SourceOneColumn::Col0), "col0");
        assert_eq!(
            get_iden_name(&TableZeroColumns::Table),
            "table_zero_columns"
        );
    }

    #[test]
    fn test_count_table_columns() {
        assert_eq!(count_table_columns::<TableZeroColumns>(), 0);
        assert_eq!(count_table_columns::<SourceOneColumn>(), 1);
        assert_eq!(count_table_columns::<SourceTwoColumns>(), 2);
    }

    #[test]
    fn test_assert_audit_table_iden_valid_one_column_good() {
        assert_audit_table_iden_valid(SourceOneColumn::Table, AuditOneColumn::Table);
    }

    #[test]
    #[should_panic]
    fn test_assert_audit_table_iden_valid_one_column_bad_missing_audit_column() {
        assert_audit_table_iden_valid(
            SourceOneColumn::Table,
            AuditOneColumnMissingAuditColumn::Table,
        );
    }

    #[test]
    #[should_panic]
    fn test_assert_audit_table_iden_valid_one_column_bad_missing_source_column() {
        assert_audit_table_iden_valid(
            SourceOneColumn::Table,
            AuditOneColumnMissingSourceColumn::Table,
        );
    }

    #[test]
    fn test_assert_audit_table_iden_valid_two_columns_good() {
        assert_audit_table_iden_valid(SourceTwoColumns::Table, AuditTwoColumns::Table);
    }

    #[test]
    fn test_find_column_with_name() {
        assert_eq!(
            find_column_with_name::<SourceOneColumn>("col0"),
            Some(SourceOneColumn::Col0)
        );

        assert_eq!(
            find_column_with_name::<SourceOneColumn>("source_one_column"),
            Some(SourceOneColumn::Table)
        );

        assert_eq!(
            find_column_with_name::<SourceTwoColumns>("col0"),
            Some(SourceTwoColumns::Col0)
        );

        assert_eq!(
            find_column_with_name::<SourceTwoColumns>("col1"),
            Some(SourceTwoColumns::Col1)
        );

        assert_eq!(
            find_column_with_name::<SourceTwoColumns>("source_two_columns"),
            Some(SourceTwoColumns::Table)
        );
    }
}
