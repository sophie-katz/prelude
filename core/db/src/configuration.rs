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

use crate::{
    entities::{configuration_entries, configuration_key_reference, configuration_type_reference},
    Error,
};
use chrono::{TimeZone, Utc};
use domain_db::{
    configuration_entry::{ConfigurationEntry, ConfigurationEntrySet},
    configuration_entry_item::ConfigurationEntryItem,
    configuration_entry_user::ConfigurationEntryUser,
    configuration_key::{ConfigurationKey, ConfigurationKeySet},
    configuration_type::{ConfigurationType, ConfigurationTypeSet},
    configuration_value::ConfigurationValue,
    constants::{
        CONFIGURATION_TYPE_BOOLEAN, CONFIGURATION_TYPE_FLOAT, CONFIGURATION_TYPE_INTEGER,
        CONFIGURATION_TYPE_STRING,
    },
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use validator::Validate;

pub async fn get_configuration_type_set(
    connection: &DatabaseConnection,
) -> Result<ConfigurationTypeSet, Error> {
    configuration_type_reference::Entity::find()
        .all(connection)
        .await?
        .into_iter()
        .map(|row| {
            let configuration_type = ConfigurationType {
                id: row.id,
                name: row.name,
                description: row.description,
            };

            configuration_type.validate()?;

            Ok((row.id, configuration_type))
        })
        .collect()
}

pub async fn get_configuration_key_set<'type_set>(
    connection: &DatabaseConnection,
    type_set: &'type_set ConfigurationTypeSet,
) -> Result<ConfigurationKeySet<'type_set>, Error> {
    configuration_key_reference::Entity::find()
        .all(connection)
        .await?
        .into_iter()
        .map(|row| {
            let key = ConfigurationKey {
                id: row.id,
                name: row.name,
                description: row.description,
                type_id: row.type_id,
                type_value: Some(
                    type_set
                        .get(&row.type_id)
                        .ok_or(Error::NoConfigurationTypeWithId(row.type_id))?,
                ),
                optional: row.optional,
                allows_multiple: row.allows_multiple,
                allows_user_override: row.allows_user_override,
            };

            key.validate()?;

            Ok((row.id, key))
        })
        .collect()
}

fn parse_configuration_value(
    configuration_type: &ConfigurationType,
    key_id: i32,
    text: &str,
) -> Result<ConfigurationValue, Error> {
    match configuration_type.name.as_str() {
        CONFIGURATION_TYPE_BOOLEAN => Ok(ConfigurationValue::Boolean(
            text.parse::<bool>()
                .map_err(|err| Error::ConfigurationValueParseErrorBoolean(key_id, err))?,
        )),
        CONFIGURATION_TYPE_INTEGER => Ok(ConfigurationValue::Integer(
            text.parse::<i32>()
                .map_err(|err| Error::ConfigurationValueParseErrorInteger(key_id, err))?,
        )),
        CONFIGURATION_TYPE_FLOAT => Ok(ConfigurationValue::Float(
            text.parse::<f64>()
                .map_err(|err| Error::ConfigurationValueParseErrorFloat(key_id, err))?,
        )),
        CONFIGURATION_TYPE_STRING => Ok(ConfigurationValue::String(text.to_string())),
        _ => Err(Error::UnsupportedConfigurationType(
            key_id,
            configuration_type.name.to_owned(),
        )),
    }
}

fn get_configuration_entry_item(
    row: &configuration_entries::Model,
    key_value: &ConfigurationKey,
) -> Result<ConfigurationEntryItem, Error> {
    Ok(ConfigurationEntryItem {
        id: row.id,
        order: row.order,
        value: parse_configuration_value(
            key_value.type_value.as_ref().unwrap(),
            row.key_id,
            &row.value,
        )?,
        create_timestamp: Utc
            .from_local_datetime(&row.create_timestamp)
            .single()
            .ok_or(Error::NoSingleDateTime(row.create_timestamp.to_owned()))?,
        deactivate_timestamp: row
            .deactivate_timestamp
            .map(|timestamp| {
                Utc.from_local_datetime(&timestamp)
                    .single()
                    .ok_or(Error::NoSingleDateTime(timestamp.to_owned()))
            })
            .transpose()?,
    })
}

pub async fn get_configuration_entry_set<'type_set, 'key_set>(
    connection: &DatabaseConnection,
    key_set: &'key_set ConfigurationKeySet<'type_set>,
    user_id: &str,
) -> Result<ConfigurationEntrySet<'type_set, 'key_set>, Error> {
    let mut result = ConfigurationEntrySet::new();

    configuration_entries::Entity::find()
        .filter(
            Condition::all()
                .add(configuration_entries::Column::DeactivateTimestamp.is_null())
                .add(
                    Condition::any().add(
                        configuration_entries::Column::UserId
                            .is_null()
                            .add(configuration_entries::Column::UserId.eq(user_id)),
                    ),
                ),
        )
        .order_by_asc(configuration_entries::Column::KeyId)
        .order_by_asc(configuration_entries::Column::Order)
        .all(connection)
        .await?
        .into_iter()
        .try_for_each(|row| -> Result<(), Error> {
            let key_value = key_set
                .get(&row.key_id)
                .ok_or(Error::NoConfigurationKeyWithId(row.key_id))?;

            let entry = result.entry(row.key_id).or_insert(ConfigurationEntry {
                key_id: row.key_id,
                key_value: Some(key_value),
                items_global: Vec::new(),
                user: None,
            });

            if let Some(user_id) = row.user_id.as_ref() {
                let user = entry.user.get_or_insert(ConfigurationEntryUser {
                    user_id: user_id.to_owned(),
                    items: Vec::new(),
                });

                user.items
                    .push(get_configuration_entry_item(&row, key_value)?);
            } else {
                entry
                    .items_global
                    .push(get_configuration_entry_item(&row, key_value)?);
            }

            Ok(())
        })?;

    Ok(result)
}
