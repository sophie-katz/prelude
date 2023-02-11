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

#[macro_use]
extern crate rocket;

use config_env::Configuration;

use rocket::{
    build,
    serde::{json::Json, Deserialize, Serialize},
    State,
};

use sea_orm::*;

use rocket::futures::executor;

use db::entities::prelude::User;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserResponse {
    id: i32,
    username: String,
}

#[get("/")]
async fn index(connection: &State<DatabaseConnection>) -> Json<Vec<UserResponse>> {
    // Json(UserResponse {
    //     id: 1,
    //     username: "admin".to_owned(),
    // })

    let connection = connection as &DatabaseConnection;

    let users = User::find()
        .all(connection)
        .await
        .unwrap()
        .into_iter()
        .map(|row| UserResponse {
            id: row.id,
            username: row.username,
        })
        .collect::<Vec<UserResponse>>();

    Json(users)
}

#[launch]
fn rocket() -> _ {
    let configuration =
        Configuration::new().expect("unable to load configuration from environment");

    let connection = executor::block_on(Database::connect(configuration.database_url))
        .expect("unable to connect to database");

    build().manage(connection).mount("/", routes![index])
}
