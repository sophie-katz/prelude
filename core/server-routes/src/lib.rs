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

// #[macro_use]
// extern crate rocket;

use rocket::{build, Build, Rocket};
use sea_orm::DatabaseConnection;

pub mod authorization;
pub mod user;

/// Build Rocket instance
pub fn rocket(db: DatabaseConnection) -> Rocket<Build> {
    build().manage(db)
    // .mount("/user", routes![user::index])
    // .mount(
    //     "/authorization",
    //     routes![authorization::login, authorization::test],
    // )
}

#[cfg(test)]
mod tests {
    use super::rocket;
    // use db::entities::user;
    // use domain_api::user::UserResponse;
    // use rocket::{http::Status, local::blocking::Client};
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase};

    // fn connect_db_mock() -> DatabaseConnection {
    //     MockDatabase::new(DatabaseBackend::Postgres)
    //         .append_query_results([vec![user::Model {
    //             id: 1,
    //             username: "admin".to_owned(),
    //             icon: None,
    //         }]])
    //         .into_connection()
    // }

    // #[test]
    // fn users_index() {
    //     let db = connect_db_mock();
    //     let client = Client::tracked(rocket(db)).expect("error creating Rocket instance");
    //     let response = client.get("/user").dispatch();
    //     assert_eq!(response.status(), Status::Ok);
    //     assert_eq!(
    //         response.into_json::<Vec<UserResponse>>().unwrap(),
    //         vec![UserResponse {
    //             id: 1,
    //             username: "admin".to_owned(),
    //             icon: None
    //         }]
    //     );
    // }
}
