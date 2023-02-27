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

//! Code specific to testing the Portobello database layer.

use crate::entities::{
    configuration_entries, configuration_entries_audit, configuration_key_reference,
    configuration_key_reference_audit, configuration_type_reference,
    configuration_type_reference_audit,
};
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;

/// Initialize the database for unit tests.
///
/// # Returns
///
/// A connection to the database which is guaranteed to be initialized.
///
/// # Errors
///
/// Returnsa any database or configuration errors.
pub async fn initialize_unit_database() -> Result<DatabaseConnection, crate::Error> {
    let connection = crate::connect_db(crate::DatabaseInstance::Unit)?;

    // Order is important here due to foreign keys

    configuration_entries::Entity::delete_many()
        .exec(&connection)
        .await?;

    configuration_key_reference::Entity::delete_many()
        .exec(&connection)
        .await?;

    configuration_type_reference::Entity::delete_many()
        .exec(&connection)
        .await?;

    configuration_entries_audit::Entity::delete_many()
        .exec(&connection)
        .await?;

    configuration_key_reference_audit::Entity::delete_many()
        .exec(&connection)
        .await?;

    configuration_type_reference_audit::Entity::delete_many()
        .exec(&connection)
        .await?;

    Ok(connection)
}
