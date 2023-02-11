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

use db::entities::prelude::User;
use domain_api::user::UserResponse;
use rocket::{serde::json::Json, State};
use sea_orm::{DatabaseConnection, EntityTrait};
use validator::Validate;

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>) -> Json<Vec<UserResponse>> {
    let db = db as &DatabaseConnection;

    let users = User::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            let user_response = UserResponse {
                id: row.id,
                username: row.username,
                icon: row.icon,
            };

            user_response
                .validate()
                .expect("user response loaded from database does not pass validation");

            user_response
        })
        .collect::<Vec<UserResponse>>();

    Json(users)
}
