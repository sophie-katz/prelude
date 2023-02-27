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

//! A utility program to seed the database with minimal usable data.

use db::{
    connect_db,
    seeding::{
        insert_configuration_entry, insert_configuration_key_reference,
        insert_configuration_type_reference,
    },
    DatabaseInstance,
};
use futures::executor;
use sea_orm::DatabaseConnection;

struct ConfigurationTypeReferenceIds {
    boolean: i32,
    integer: i32,
    float: i32,
    string: i32,
}

struct ConfigurationKeyReferenceIds {
    system_enabled_code: i32,
    system_enabled_dashboard: i32,
    system_enabled_deploy: i32,
    system_enabled_document: i32,
    system_enabled_ticket: i32,
}

async fn seed_configuration_type_reference(
    connection: &DatabaseConnection,
) -> Result<ConfigurationTypeReferenceIds, db::Error> {
    Ok(ConfigurationTypeReferenceIds {
        boolean: insert_configuration_type_reference(connection, "boolean", "A true/false value")
            .await?,
        integer: insert_configuration_type_reference(
            connection,
            "integer",
            "A signed integer number",
        )
        .await?,
        float: insert_configuration_type_reference(connection, "float", "A floating-point number")
            .await?,
        string: insert_configuration_type_reference(connection, "string", "A string value").await?,
    })
}

async fn seed_configuration_key_reference(
    connection: &DatabaseConnection,
    configuration_type_reference_ids: &ConfigurationTypeReferenceIds,
) -> Result<ConfigurationKeyReferenceIds, db::Error> {
    Ok(ConfigurationKeyReferenceIds {
        system_enabled_code: insert_configuration_key_reference(
            connection,
            "system.enabled.code",
            "Whether or not the Code system is enabled",
            configuration_type_reference_ids.boolean,
            false,
            false,
            false,
        )
        .await?,
        system_enabled_dashboard: insert_configuration_key_reference(
            connection,
            "system.enabled.dashboard",
            "Whether or not the Dashboard system is enabled",
            configuration_type_reference_ids.boolean,
            false,
            false,
            false,
        )
        .await?,
        system_enabled_deploy: insert_configuration_key_reference(
            connection,
            "system.enabled.deploy",
            "Whether or not the Deploy system is enabled",
            configuration_type_reference_ids.boolean,
            false,
            false,
            false,
        )
        .await?,
        system_enabled_document: insert_configuration_key_reference(
            connection,
            "system.enabled.document",
            "Whether or not the Document system is enabled",
            configuration_type_reference_ids.boolean,
            false,
            false,
            false,
        )
        .await?,
        system_enabled_ticket: insert_configuration_key_reference(
            connection,
            "system.enabled.ticket",
            "Whether or not the Ticket system is enabled",
            configuration_type_reference_ids.boolean,
            false,
            false,
            false,
        )
        .await?,
    })
}

async fn seed_configuration_entries(
    connection: &DatabaseConnection,
    configuration_key_reference_ids: &ConfigurationKeyReferenceIds,
) -> Result<(), db::Error> {
    insert_configuration_entry(
        connection,
        configuration_key_reference_ids.system_enabled_code,
        1,
        None,
        "true",
    )
    .await?;

    insert_configuration_entry(
        connection,
        configuration_key_reference_ids.system_enabled_dashboard,
        1,
        None,
        "true",
    )
    .await?;

    insert_configuration_entry(
        connection,
        configuration_key_reference_ids.system_enabled_deploy,
        1,
        None,
        "true",
    )
    .await?;

    insert_configuration_entry(
        connection,
        configuration_key_reference_ids.system_enabled_document,
        1,
        None,
        "true",
    )
    .await?;

    insert_configuration_entry(
        connection,
        configuration_key_reference_ids.system_enabled_ticket,
        1,
        None,
        "true",
    )
    .await?;

    Ok(())
}

async fn seed(connection: &DatabaseConnection) -> Result<(), db::Error> {
    let configuration_type_reference_ids = seed_configuration_type_reference(connection).await?;

    let configuration_key_reference_ids =
        seed_configuration_key_reference(connection, &configuration_type_reference_ids).await?;

    seed_configuration_entries(connection, &configuration_key_reference_ids).await?;

    Ok(())
}

fn main() {
    let db = connect_db(DatabaseInstance::Development).expect("unable to connect to database");

    executor::block_on(seed(&db)).expect("error while seeding");
}
