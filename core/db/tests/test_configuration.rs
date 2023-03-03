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

#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use db::{
    queries::configuration::{
        get_all_configuration_entries, get_all_configuration_keys, get_all_configuration_types,
    },
    seeding::{
        insert_configuration_entry, insert_configuration_key_reference,
        insert_configuration_type_reference,
    },
    testing::initialize_unit_database,
};
use serial_test::serial;

#[async_std::test]
#[serial]
async fn test_get_all_configuration_types() -> Result<(), db::Error> {
    let connection = initialize_unit_database().await?;

    let types = get_all_configuration_types(&connection).await?;

    assert_eq!(types.len(), 0);

    insert_configuration_type_reference(&connection, "boolean", "A true/false value").await?;
    insert_configuration_type_reference(&connection, "integer", "A signed integer value").await?;

    let types = get_all_configuration_types(&connection).await?;

    assert_eq!(types.len(), 2);

    assert!(types[0].id > 0);
    assert_eq!(types[0].name, "boolean");
    assert_eq!(types[0].description, "A true/false value");

    assert!(types[1].id > 0);
    assert_eq!(types[1].name, "integer");
    assert_eq!(types[1].description, "A signed integer value");

    assert_eq!(types[1].id, types[0].id + 1);

    Ok(())
}

#[async_std::test]
#[serial]
async fn test_get_all_configuration_keys() -> Result<(), db::Error> {
    let connection = initialize_unit_database().await?;

    let types = get_all_configuration_types(&connection).await?;

    assert_eq!(types.len(), 0);

    let boolean_id =
        insert_configuration_type_reference(&connection, "boolean", "A true/false value").await?;

    let types = get_all_configuration_types(&connection).await?;

    let keys = get_all_configuration_keys(&connection, &types).await?;

    assert_eq!(keys.len(), 0);

    insert_configuration_key_reference(
        &connection,
        "systems.enabled.code",
        "Whether or not the Code system is enabled",
        boolean_id,
        false,
        false,
        false,
    )
    .await?;

    insert_configuration_key_reference(
        &connection,
        "systems.enabled.ticket",
        "Whether or not the Ticket system is enabled",
        boolean_id,
        false,
        false,
        false,
    )
    .await?;

    let keys = get_all_configuration_keys(&connection, &types).await?;

    assert_eq!(keys.len(), 2);

    assert!(keys[0].id > 0);
    assert_eq!(keys[0].name, "systems.enabled.code");
    assert_eq!(
        keys[0].description,
        "Whether or not the Code system is enabled"
    );
    assert_eq!(keys[0].configuration_type.id, boolean_id);
    assert_eq!(keys[0].configuration_type.name, "boolean");
    assert_eq!(keys[0].configuration_type.description, "A true/false value");
    assert!(!keys[0].optional);
    assert!(!keys[0].allows_multiple);
    assert!(!keys[0].allows_user_override);

    assert!(keys[1].id > 0);
    assert_eq!(keys[1].name, "systems.enabled.ticket");
    assert_eq!(
        keys[1].description,
        "Whether or not the Ticket system is enabled"
    );
    assert_eq!(keys[1].configuration_type.id, boolean_id);
    assert_eq!(keys[1].configuration_type.name, "boolean");
    assert_eq!(keys[1].configuration_type.description, "A true/false value");
    assert!(!keys[1].optional);
    assert!(!keys[1].allows_multiple);
    assert!(!keys[1].allows_user_override);

    assert_eq!(keys[1].id, keys[0].id + 1);

    Ok(())
}

#[async_std::test]
#[serial]
async fn test_get_all_configuration_entries() -> Result<(), db::Error> {
    let connection = initialize_unit_database().await?;

    let types = get_all_configuration_types(&connection).await?;

    assert_eq!(types.len(), 0);

    let boolean_id =
        insert_configuration_type_reference(&connection, "boolean", "A true/false value").await?;

    let types = get_all_configuration_types(&connection).await?;

    let keys = get_all_configuration_keys(&connection, &types).await?;

    assert_eq!(keys.len(), 0);

    let systems_enabled_code_id = insert_configuration_key_reference(
        &connection,
        "systems.enabled.code",
        "Whether or not the Code system is enabled",
        boolean_id,
        false,
        false,
        false,
    )
    .await?;

    let keys = get_all_configuration_keys(&connection, &types).await?;

    let entries = get_all_configuration_entries(&connection, &keys, None).await?;

    assert_eq!(entries.len(), 0);

    insert_configuration_entry(&connection, systems_enabled_code_id, 1, None, "true").await?;

    let entries = get_all_configuration_entries(&connection, &keys, None).await?;

    assert_eq!(entries.len(), 1);

    assert_eq!(entries[0].key.id, systems_enabled_code_id);
    assert_eq!(entries[0].key.name, "systems.enabled.code");
    assert_eq!(
        entries[0].key.description,
        "Whether or not the Code system is enabled"
    );
    assert_eq!(entries[0].key.configuration_type.id, boolean_id);
    assert_eq!(entries[0].key.configuration_type.name, "boolean");
    assert_eq!(
        entries[0].key.configuration_type.description,
        "A true/false value"
    );
    assert!(!entries[0].key.optional);
    assert!(!entries[0].key.allows_multiple);
    assert!(!entries[0].key.allows_user_override);
    assert_eq!(entries[0].items_global.len(), 1);
    assert!(entries[0].items_global[0].id > 0);
    assert_eq!(entries[0].items_global[0].value.as_boolean, Some(true));
    assert_eq!(entries[0].items_global[0].value.as_integer, None);
    assert_eq!(entries[0].items_global[0].value.as_float, None);
    assert_eq!(entries[0].items_global[0].value.as_string, None);
    assert_eq!(entries[0].user, None);

    Ok(())
}
