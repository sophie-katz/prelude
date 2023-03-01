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

use crate::{
    entities::{configuration_entries, configuration_key_reference, configuration_type_reference},
    Error,
};
use domain_api::configuration::{
    ConfigurationEntryItemResponse, ConfigurationEntryResponse, ConfigurationEntrySetResponse,
    ConfigurationEntryUserResponse, ConfigurationKeyResponse, ConfigurationKeySetResponse,
    ConfigurationTypeResponse, ConfigurationTypeSetResponse, ConfigurationValueResponse,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Select,
};
use std::collections::{hash_map::Entry, HashMap};
use validator::Validate;

/// Get all configuration types from the database
///
/// # Arguments
///
/// * `connection` - The database connection
///
/// # Returns
///
/// The set of configuration types.
///
/// # Errors
///
/// Returns any database errors.
pub async fn get_all_configuration_types(
    connection: &DatabaseConnection,
) -> Result<ConfigurationTypeSetResponse, Error> {
    configuration_type_reference::Entity::find()
        .order_by_asc(configuration_type_reference::Column::Id)
        .filter(configuration_type_reference::Column::DeactivateTimestamp.is_null())
        .all(connection)
        .await?
        .into_iter()
        .map(|row| {
            let configuration_type_response = ConfigurationTypeResponse {
                id: row.id,
                name: row.name,
                description: row.description,
            };

            configuration_type_response.validate()?;

            Ok(configuration_type_response)
        })
        .collect::<Result<ConfigurationTypeSetResponse, Error>>()
}

/// Get all configuration keys from the database
///
/// # Arguments
///
/// * `connection` - The database connection
/// * `type_set` - The set of already loaded configuration types
///
/// # Returns
///
/// The set of configuration keys.
///
/// # Errors
///
/// Returns any database errors. If there is a configuration type id referenced
/// that is not in the set of configuration types, an error is returned.
pub async fn get_all_configuration_keys(
    connection: &DatabaseConnection,
    type_set: &ConfigurationTypeSetResponse,
) -> Result<ConfigurationKeySetResponse, Error> {
    configuration_key_reference::Entity::find()
        .order_by_asc(configuration_key_reference::Column::Id)
        .filter(configuration_key_reference::Column::DeactivateTimestamp.is_null())
        .all(connection)
        .await?
        .into_iter()
        .map(|row| {
            let configuration_key_response = ConfigurationKeyResponse {
                id: row.id,
                name: row.name,
                description: row.description,
                configuration_type: type_set
                    .iter()
                    .find(|t| t.id == row.type_id)
                    .ok_or(Error::ConfigurationTypeNotFound(row.type_id))?
                    .clone(),
                optional: row.optional,
                allows_multiple: row.allows_multiple,
                allows_user_override: row.allows_user_override,
            };

            configuration_key_response.validate()?;

            Ok(configuration_key_response)
        })
        .collect::<Result<ConfigurationKeySetResponse, Error>>()
}

/// Get all configuration entries from the database
///
/// # Arguments
///
/// * `connection` - The database connection
/// * `type_set` - The set of already loaded configuration types
/// * `key_set` - The set of already loaded configuration keys
///
/// # Returns
///
/// The set of configuration entries.
///
/// # Errors
///
/// Returns any database errors. If there is a configuration type id referenced
/// that is not in the set of configuration types, an error is returned. This is
/// true for configuration keys as well.
pub async fn get_all_configuration_entries(
    connection: &DatabaseConnection,
    key_set: &ConfigurationKeySetResponse,
    user_id: Option<&str>,
) -> Result<ConfigurationEntrySetResponse, Error> {
    // Build query
    let query = build_configuration_entries_query(user_id);

    // Create cache
    let mut configuration_entries_map: HashMap<i32, ConfigurationEntryResponse> = HashMap::new();

    // Iterate over query response rows
    for row in query.all(connection).await? {
        // Find the entry in the cache
        let configuration_entries_map_entry = configuration_entries_map.entry(row.key_id);

        // Populate it if it does not already exist
        let configuration_entry = match configuration_entries_map_entry {
            Entry::Occupied(configuration_entry_occupied) => {
                configuration_entry_occupied.into_mut()
            }
            Entry::Vacant(configuration_entry_vacant) => {
                configuration_entry_vacant.insert(ConfigurationEntryResponse {
                    key: key_set
                        .iter()
                        .find(|k| k.id == row.key_id)
                        .ok_or(Error::ConfigurationKeyNotFound(row.key_id))
                        .map(|x| x.clone())?,
                    items_global: Vec::new(),
                    user: None,
                })
            }
        };

        // Create entry item from parsed text value
        let entry_item = ConfigurationEntryItemResponse {
            id: row.id,
            value: parse_configuration_value(
                &row.value,
                &configuration_entry.key.configuration_type,
            )?,
        };

        // Push entry item into correct vector in entry
        if let Some(user_id) = row.user_id {
            let user =
                configuration_entry
                    .user
                    .get_or_insert_with(|| ConfigurationEntryUserResponse {
                        user_id,
                        items: Vec::new(),
                    });

            user.items.push(entry_item);
        } else {
            configuration_entry.items_global.push(entry_item);
        }
    }

    // Return cache as vector
    Ok(configuration_entries_map
        .into_values()
        .collect::<ConfigurationEntrySetResponse>())
}

/// Build a query that selects all configuration entries from the database for
/// a given user.
///
/// # Arguments
///
/// * `user_id` - The user id to select the configuration entries for. If this
///               value is null, return only global configuration entries.
fn build_configuration_entries_query(
    user_id: Option<&str>,
) -> Select<configuration_entries::Entity> {
    let mut query = configuration_entries::Entity::find()
        .order_by_asc(configuration_entries::Column::KeyId)
        .order_by_asc(configuration_entries::Column::OrderIndex)
        .filter(configuration_entries::Column::DeactivateTimestamp.is_null());

    if let Some(user_id) = user_id {
        query = query.filter(
            Condition::any()
                .add(configuration_entries::Column::UserId.is_null())
                .add(configuration_entries::Column::UserId.eq(user_id)),
        );
    } else {
        query = query.filter(configuration_entries::Column::UserId.is_null());
    }

    query
}

/// Parse a configuration value from a string.
///
/// # Arguments
///
/// * `text` - The text to parse
/// * `configuration_type` - The configuration type to parse the text as
///
/// # Returns
///
/// A configuration value response.
///
/// # Errors
///
/// Returns an error if the text cannot be parsed as the given configuration.
fn parse_configuration_value(
    text: &str,
    configuration_type: &ConfigurationTypeResponse,
) -> Result<ConfigurationValueResponse, Error> {
    match configuration_type.name.as_str() {
        "boolean" => Ok(ConfigurationValueResponse {
            as_boolean: match text {
                "true" => Ok(Some(true)),
                "false" => Ok(Some(false)),
                _ => Err(Error::ConfigurationValueParseErrorBoolean(text.to_owned())),
            }?,
            ..Default::default()
        }),
        "integer" => Ok(ConfigurationValueResponse {
            as_integer: Some(text.parse::<i64>()?),
            ..Default::default()
        }),
        "float" => Ok(ConfigurationValueResponse {
            as_float: Some(text.parse::<f64>()?),
            ..Default::default()
        }),
        "string" => Ok(ConfigurationValueResponse {
            as_string: Some(text.to_owned()),
            ..Default::default()
        }),
        _ => Err(Error::ConfigurationTypeNotFound(configuration_type.id)),
    }
}
