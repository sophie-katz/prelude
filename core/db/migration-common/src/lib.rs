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
        .collect::<HashSet<String>>();

    // Assert that the audit table contains all the source table identifiers
    for source_table_iden in SourceTableEnum::iter().map(|x| -> String {
        let mut result = String::new();
        x.unquoted(&mut result);
        result
    }) {
        assert!(audit_table_iden_set.contains(&source_table_iden));
    }

    // Assert that the audit table contains all the standard audit table
    // identifiers
    for audit_table_iden in AUDIT_TABLE_COLUMNS {
        assert!(audit_table_iden_set.contains(audit_table_iden));
    }

    // Assert that the audit table contains no extra identifiers
    assert_eq!(
        audit_table_iden_set.len(),
        SourceTableEnum::iter().count() + AUDIT_TABLE_COLUMNS.len()
    );
}
