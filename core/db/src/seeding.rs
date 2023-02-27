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
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Utility functions to help with seeding databases.

use crate::entities::{
    configuration_entries, configuration_key_reference, configuration_type_reference,
};
use sea_orm::{DatabaseConnection, EntityTrait, Set};

/// Inserts a configuration type reference into the database.
///
/// # Arguments
///
/// * `connection` - The database connection to use.
/// * `name` - The name of the configuration type.
/// * `description` - The description of the configuration type.
///
/// # Returns
///
/// The id of the newly inserted configuration type reference.
///
/// # Errors
///
/// Returns any database errors.
pub async fn insert_configuration_type_reference(
    connection: &DatabaseConnection,
    name: &str,
    description: &str,
) -> Result<i32, crate::Error> {
    Ok(
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set(name.to_owned()),
            description: Set(description.to_owned()),
            ..Default::default()
        })
        .exec(connection)
        .await?
        .last_insert_id,
    )
}

/// Inserts a configuration key reference into the database.
///
/// # Arguments
///
/// * `connection` - The database connection to use.
/// * `name` - The name of the configuration key.
/// * `description` - The description of the configuration key.
/// * `type_id` - The id of the configuration type.
/// * `optional` - Whether the configuration key is optional.
/// * `allows_multiple` - Whether the configuration key allows multiple values.
/// * `allows_user_override` - Whether the configuration key allows user
///                            overrides.
///
/// # Returns
///
/// The id of the newly inserted configuration key reference.
///
/// # Errors
///
/// Returns any database errors.
pub async fn insert_configuration_key_reference(
    connection: &DatabaseConnection,
    name: &str,
    description: &str,
    type_id: i32,
    optional: bool,
    allows_multiple: bool,
    allows_user_override: bool,
) -> Result<i32, crate::Error> {
    Ok(
        configuration_key_reference::Entity::insert(configuration_key_reference::ActiveModel {
            name: Set(name.to_owned()),
            description: Set(description.to_owned()),
            type_id: Set(type_id),
            optional: Set(optional),
            allows_multiple: Set(allows_multiple),
            allows_user_override: Set(allows_user_override),
            ..Default::default()
        })
        .exec(connection)
        .await?
        .last_insert_id,
    )
}

/// Inserts a configuration entry into the database.
///
/// # Arguments
///
/// * `connection` - The database connection to use.
/// * `key_id` - The id of the configuration key.
/// * `user_id` - An optional user id to denote that the entry a user override
///               on the global value.
/// * `value` - The string representation of the configuratin entry's value.
///
/// # Errors
///
/// Returns any database errors.
pub async fn insert_configuration_entry(
    connection: &DatabaseConnection,
    key_id: i32,
    order_index: i32,
    user_id: Option<&str>,
    value: &str,
) -> Result<(), crate::Error> {
    configuration_entries::Entity::insert(configuration_entries::ActiveModel {
        key_id: Set(key_id),
        order_index: Set(order_index),
        value: Set(value.to_owned()),
        user_id: Set(user_id.map(|x| x.to_owned())),
        ..Default::default()
    })
    .exec(connection)
    .await?;

    Ok(())
}
