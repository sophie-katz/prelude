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

pub mod keys;
pub mod types;

use db::entities::{
    configuration_entries, configuration_key_reference, configuration_type_reference,
};
use domain_api::configuration::{
    ConfigurationEntrySetResponse, ConfigurationKeyResponse, ConfigurationKeySetResponse,
    ConfigurationTypeResponse,
};
use rocket::{serde::json::Json, State};
use sea_orm::{DatabaseConnection, EntityTrait};
use validator::Validate;

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>) -> Json<ConfigurationEntrySetResponse> {
    let connection = db as &DatabaseConnection;

    // let configuration_entries = configuration_entries::Entity::find()
    //     .find_also_related(configuration_key_reference::Entity)
    //     .all(connection)
    //     .await
    //     .unwrap()
    //     .into_iter()
    //     .map(|row| {
    //         // let row_type = row_type.unwrap();

    //         // let configuration_key_response = ConfigurationKeyResponse {
    //         //     id: row_key.id,
    //         //     name: row_key.name,
    //         //     description: row_key.description,
    //         //     configuration_type: ConfigurationTypeResponse {
    //         //         id: row_type.id,
    //         //         name: row_type.name,
    //         //         description: row_type.description,
    //         //     },
    //         //     optional: row_key.optional,
    //         //     allows_multiple: row_key.allows_multiple,
    //         //     allows_user_override: row_key.allows_user_override,
    //         // };

    //         // configuration_key_response
    //         //     .validate()
    //         //     .expect("configuration key response loaded from database does not pass validation");

    //         // configuration_key_response

    //         todo!()
    //     })
    //     .collect::<ConfigurationKeySetResponse>();

    // Json(configuration_keys)

    configuration_entries::Entity::find()
        .join(JoinType::InnerJoin)

    todo!()
}
