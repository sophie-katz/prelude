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

use db::{
    connect_db,
    entities::{configuration_reference, configuration_type_reference},
    DatabaseInstance,
};
use futures::executor;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};

struct ConfigurationTypeReferenceIds {
    boolean: i32,
    integer: i32,
    float: i32,
    string: i32,
}

async fn seed_configuration_type_reference(
    connection: &DatabaseConnection,
) -> Result<ConfigurationTypeReferenceIds, DbErr> {
    let boolean =
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set("boolean".to_owned()),
            description: Set("A true/false value".to_owned()),
            ..Default::default()
        })
        .exec(connection)
        .await?
        .last_insert_id;

    let integer =
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set("integer".to_owned()),
            description: Set("A signed integer number".to_owned()),
            ..Default::default()
        })
        .exec(connection)
        .await?
        .last_insert_id;

    let float =
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set("float".to_owned()),
            description: Set("A floating-point number".to_owned()),
            ..Default::default()
        })
        .exec(connection)
        .await?
        .last_insert_id;

    let string =
        configuration_type_reference::Entity::insert(configuration_type_reference::ActiveModel {
            name: Set("string".to_owned()),
            description: Set("A string value".to_owned()),
            ..Default::default()
        })
        .exec(connection)
        .await?
        .last_insert_id;

    Ok(ConfigurationTypeReferenceIds {
        boolean,
        integer,
        float,
        string,
    })
}

async fn seed_configuration_reference(
    connection: &DatabaseConnection,
    configuration_type_reference_ids: &ConfigurationTypeReferenceIds,
) -> Result<(), DbErr> {
    configuration_reference::Entity::insert(configuration_reference::ActiveModel {
        name: Set("system.enabled.code".to_owned()),
        description: Set("Whether or not the Code system is enabled".to_owned()),
        type_id: Set(configuration_type_reference_ids.boolean),
        optional: Set(false),
        allows_multiple: Set(false),
        allows_user_override: Set(false),
        ..Default::default()
    })
    .exec(connection)
    .await?;

    configuration_reference::Entity::insert(configuration_reference::ActiveModel {
        name: Set("system.enabled.dashboard".to_owned()),
        description: Set("Whether or not the Dashboard system is enabled".to_owned()),
        type_id: Set(configuration_type_reference_ids.boolean),
        optional: Set(false),
        allows_multiple: Set(false),
        allows_user_override: Set(false),
        ..Default::default()
    })
    .exec(connection)
    .await?;

    configuration_reference::Entity::insert(configuration_reference::ActiveModel {
        name: Set("system.enabled.deploy".to_owned()),
        description: Set("Whether or not the Deploy system is enabled".to_owned()),
        type_id: Set(configuration_type_reference_ids.boolean),
        optional: Set(false),
        allows_multiple: Set(false),
        allows_user_override: Set(false),
        ..Default::default()
    })
    .exec(connection)
    .await?;

    configuration_reference::Entity::insert(configuration_reference::ActiveModel {
        name: Set("system.enabled.document".to_owned()),
        description: Set("Whether or not the Document system is enabled".to_owned()),
        type_id: Set(configuration_type_reference_ids.boolean),
        optional: Set(false),
        allows_multiple: Set(false),
        allows_user_override: Set(false),
        ..Default::default()
    })
    .exec(connection)
    .await?;

    configuration_reference::Entity::insert(configuration_reference::ActiveModel {
        name: Set("system.enabled.ticket".to_owned()),
        description: Set("Whether or not the Ticket system is enabled".to_owned()),
        type_id: Set(configuration_type_reference_ids.boolean),
        optional: Set(false),
        allows_multiple: Set(false),
        allows_user_override: Set(false),
        ..Default::default()
    })
    .exec(connection)
    .await?;

    Ok(())
}

async fn seed(connection: &DatabaseConnection) -> Result<(), DbErr> {
    let configuration_type_reference_ids = seed_configuration_type_reference(connection).await?;

    seed_configuration_reference(connection, &configuration_type_reference_ids).await?;

    Ok(())
}

fn main() {
    let db = connect_db(DatabaseInstance::Development).expect("unable to connect to database");

    executor::block_on(seed(&db)).expect("error while seeding");
}
