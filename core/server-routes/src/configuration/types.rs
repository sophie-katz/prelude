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

use db::entities::configuration_type_reference;
use domain_api::configuration::{ConfigurationTypeResponse, ConfigurationTypeSetResponse};
use rocket::{serde::json::Json, State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use validator::Validate;

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>) -> Json<ConfigurationTypeSetResponse> {
    let connection = db as &DatabaseConnection;

    let configuration_types = configuration_type_reference::Entity::find()
        .order_by_asc(configuration_type_reference::Column::Id)
        .filter(configuration_type_reference::Column::DeactivateTimestamp.is_null())
        .all(connection)
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            let configuration_type_response = ConfigurationTypeResponse {
                id: row.id,
                name: row.name,
                description: row.description,
            };

            configuration_type_response.validate().expect(
                "configuration type response loaded from database does not pass validation",
            );

            configuration_type_response
        })
        .collect::<ConfigurationTypeSetResponse>();

    Json(configuration_types)
}
