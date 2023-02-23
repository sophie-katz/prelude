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

pub mod configuration;
mod entities;

use chrono::NaiveDateTime;
use config_env::Configuration;
use futures::executor;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use std::{
    error,
    fmt::{self, Display},
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};
use validator::ValidationErrors;

/// Error type for this crate
#[derive(Debug)]
pub enum Error {
    /// No configuration type with the given id
    NoConfigurationTypeWithId(i32),
    NoConfigurationKeyWithId(i32),
    NoSingleDateTime(NaiveDateTime),
    UnsupportedConfigurationType(i32, String),
    ConfigurationValueParseErrorBoolean(i32, ParseBoolError),
    ConfigurationValueParseErrorInteger(i32, ParseIntError),
    ConfigurationValueParseErrorFloat(i32, ParseFloatError),
    /// Wrapper for config-env errors
    ConfigEnvError(config_env::Error),
    /// Wrapper for SeaORM errors
    SeaORMDbErr(DbErr),
    ValidatorValidationErrors(ValidationErrors),
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

impl From<ValidationErrors> for Error {
    fn from(value: ValidationErrors) -> Self {
        Self::ValidatorValidationErrors(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoConfigurationTypeWithId(id) => {
                write!(f, "no configuration type with id {}", id)
            }
            Error::NoConfigurationKeyWithId(id) => {
                write!(f, "no configuration key with id {}", id)
            }
            Error::NoSingleDateTime(datetime) => {
                write!(f, "no single datetime in timezone for {}", datetime)
            }
            Error::UnsupportedConfigurationType(id, name) => {
                write!(
                    f,
                    "unsupported configuration type {:#?} (configuration key id: {})",
                    name, id
                )
            }
            Error::ConfigurationValueParseErrorBoolean(id, err) => {
                write!(
                    f,
                    "error parsing boolean configuration value (configuration key id: {}): {}",
                    id, err
                )
            }
            Error::ConfigurationValueParseErrorInteger(id, err) => {
                write!(
                    f,
                    "error parsing integer configuration value (configuration key id: {}): {}",
                    id, err
                )
            }
            Error::ConfigurationValueParseErrorFloat(id, err) => {
                write!(
                    f,
                    "error parsing float configuration value (configuration key id: {}): {}",
                    id, err
                )
            }
            Error::ConfigEnvError(err) => write!(f, "{err}"),
            Error::SeaORMDbErr(err) => write!(f, "{err}"),
            Error::ValidatorValidationErrors(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for Error {}

#[derive(Debug)]
pub enum DatabaseInstance {
    Development,
    Unit,
}

impl DatabaseInstance {
    pub fn as_name(self) -> &'static str {
        match self {
            DatabaseInstance::Development => "portobello_dev",
            DatabaseInstance::Unit => "portobello_unit",
        }
    }
}

/// Connect to the Portobello database
///
/// # Errors
///
/// If there is an issue loading the database connection configuration or in
/// connecting to the database, an error will be returned.
pub fn connect_db(database_instance: DatabaseInstance) -> Result<DatabaseConnection, Error> {
    let configuration = Configuration::new()?;

    Ok(executor::block_on(Database::connect(format!(
        "postgres://{}:{}@{}:{}/{}",
        configuration.postgres_user,
        configuration.postgres_password,
        configuration.postgres_host,
        configuration.postgres_port,
        database_instance.as_name(),
    )))?)
}

// #[cfg(test)]
// mod tests {
//     use super::{connect_db, connect_mock_db, Error};

//     // #[test]
//     // fn test_connect_db_development() -> Result<(), Error> {
//     //     connect_db(Database::Development)?;

//     //     Ok(())
//     // }

//     // #[async_std::test]
//     // async fn test_find_configuration_type_reference_one() -> Result<(), DbErr> {
//     //     let db = MockDatabase::new(DatabaseBackend::Postgres)
//     //         .append_query_results([vec![configuration_type_reference::Model {
//     //             id: 1,
//     //             name: "boolean".to_owned(),
//     //             description: "True or false value".to_owned(),
//     //         }]])
//     //         // .append_query_results(vec![configuration_type_reference::Model {
//     //         //     id: 2,
//     //         //     name: "integer".to_owned(),
//     //         //     description: "Signed integer value".to_owned(),
//     //         // }])
//     //         // .append_query_results(vec![configuration_type_reference::Model {
//     //         //     id: 3,
//     //         //     name: "string".to_owned(),
//     //         //     description: "String value".to_owned(),
//     //         // }])
//     //         // .append_query_results([vec![configuration_reference::Model {
//     //         //     id: 1,
//     //         //     name: "booleanRequiredSingleGlobal".to_owned(),
//     //         //     description: "A boolean value that is required, cannot have multiple values, and cannot be overridden by users".to_owned(),
//     //         //     type_id: 1,
//     //         //     optional: false,
//     //         //     allows_multiple: false,
//     //         //     allows_user_override: false,
//     //         // }]])
//     //         .into_connection();

//     //     assert_eq!(
//     //         ConfigurationTypeReference::find()
//     //             .all(&db)
//     //             .await?
//     //             .into_iter()
//     //             .collect::<Vec<user::Model>>(),
//     //         vec![user::Model {
//     //             id: 1,
//     //             username: "admin".to_owned(),
//     //             icon: None
//     //         }]
//     //     );

//     //     Ok(())
//     // }
// }
