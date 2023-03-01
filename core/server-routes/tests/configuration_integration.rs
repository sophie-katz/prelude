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
    seeding::{
        insert_configuration_entry, insert_configuration_key_reference,
        insert_configuration_type_reference,
    },
    testing::initialize_unit_database,
};
use rocket::{http::Status, local::asynchronous::Client};
use serde_json::json;
use serial_test::serial;

#[async_std::test]
#[serial]
async fn test_index() -> Result<(), db::Error> {
    let connection = initialize_unit_database().await?;

    let boolean_id =
        insert_configuration_type_reference(&connection, "boolean", "A true/false value").await?;

    let systems_enabled_code_id = insert_configuration_key_reference(
        &connection,
        "systems.enabled.code",
        "Whether the Code system is enabled or not",
        boolean_id,
        false,
        false,
        false,
    )
    .await?;

    let entry_id =
        insert_configuration_entry(&connection, systems_enabled_code_id, 1, None, "true").await?;

    let client = Client::tracked(server_routes::rocket(connection))
        .await
        .expect("error creating Rocket instance");

    let response = client.get("/configuration").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_json::<serde_json::Value>().await.unwrap(),
        json!([
            {
                "key": {
                    "id": systems_enabled_code_id,
                    "name": "systems.enabled.code",
                    "description": "Whether the Code system is enabled or not",
                    "type": {
                        "id": boolean_id,
                        "name": "boolean",
                        "description": "A true/false value"
                    },
                    "optional": false,
                    "allowsMultiple": false,
                    "allowsUserOverride": false
                },
                "itemsGlobal": [
                    {
                        "id": entry_id,
                        "value": {
                            "asBoolean": true,
                            "asInteger": null,
                            "asFloat": null,
                            "asString": null
                        }
                    }
                ],
                "user": null
            }
        ])
    );

    Ok(())
}
