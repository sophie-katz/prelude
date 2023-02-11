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
use rocket::{build, futures::executor, Build, Rocket};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{
    error,
    fmt::{self, Display},
};

pub mod user;

/// Error type for this crate
#[derive(Debug)]
pub enum Error {
    /// Wrapper for config-env errors
    ConfigEnvError(config_env::Error),
    /// Wrapper for SeaORM errors
    SeaORMDbErr(DbErr),
}

impl From<config_env::Error> for Error {
    fn from(value: config_env::Error) -> Self {
        Self::ConfigEnvError(value)
    }
}

impl From<DbErr> for Error {
    fn from(value: DbErr) -> Self {
        Self::SeaORMDbErr(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConfigEnvError(err) => write!(f, "{err}"),
            Error::SeaORMDbErr(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for Error {}

/// Connect to the Portobello database
///
/// # Errors
///
/// If there is an issue loading the database connection configuration or in
/// connecting to the database, an error will be returned.
pub fn connect_db() -> Result<DatabaseConnection, Error> {
    let configuration = Configuration::new()?;

    Ok(executor::block_on(Database::connect(
        configuration.database_url,
    ))?)
}

/// Build Rocket instance
pub fn rocket(db: DatabaseConnection) -> Rocket<Build> {
    build().manage(db).mount("/user", routes![user::index])
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use db::entities::user;
    use domain_api::user::UserResponse;
    use rocket::{http::Status, local::blocking::Client};
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase};

    fn connect_db_mock() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![user::Model {
                id: 1,
                username: "admin".to_owned(),
                icon: None,
            }]])
            .into_connection()
    }

    #[test]
    fn users_index() {
        let db = connect_db_mock();
        let client = Client::tracked(rocket(db)).expect("error creating Rocket instance");
        let response = client.get("/user").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_json::<Vec<UserResponse>>().unwrap(),
            vec![UserResponse {
                id: 1,
                username: "admin".to_owned(),
                icon: None
            }]
        );
    }
}
