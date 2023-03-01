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

//! The Portobello database layer.

#![allow(missing_docs, incomplete_features)]
#![feature(async_fn_in_trait)]

pub mod entities;
pub mod queries;
pub mod seeding;
pub mod testing;

use chrono::Utc;
use config_env::Configuration;
use futures::executor;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use std::{
    error,
    fmt::{self, Display},
    num::{ParseFloatError, ParseIntError},
};
use validator::ValidationErrors;

/// An alias for the datetime type used for timestamps in the database.
pub type DateTime = chrono::DateTime<Utc>;

/// Error type for this crate
#[derive(Debug)]
pub enum Error {
    /// A configuration type was not found for the given id
    ConfigurationTypeNotFound(i32),
    /// A configuration key was not found for the given id
    ConfigurationKeyNotFound(i32),
    /// Could not parse a boolean configuration value
    ConfigurationValueParseErrorBoolean(String),
    /// Wrapper for integer parsing errors
    NumParseIntError(ParseIntError),
    /// Wrapper for float parsing errors
    NumParseFloatError(ParseFloatError),
    /// Wrapper for config-env errors
    ConfigEnvError(config_env::Error),
    /// Wrapper for SeaORM errors
    SeaORMDbErr(DbErr),
    /// Wrapper for validator errors
    ValidatorValidationErrors(ValidationErrors),
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::NumParseIntError(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Self::NumParseFloatError(value)
    }
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
            Error::ConfigurationTypeNotFound(id) => {
                write!(f, "configuration type not found for id {id}")
            }
            Error::ConfigurationKeyNotFound(id) => {
                write!(f, "configuration key not found for id {id}")
            }
            Error::ConfigurationValueParseErrorBoolean(text) => {
                write!(f, "could not parse {text:#?} as a boolean")
            }
            Error::NumParseIntError(err) => write!(f, "{err}"),
            Error::NumParseFloatError(err) => write!(f, "{err}"),
            Error::ConfigEnvError(err) => write!(f, "{err}"),
            Error::SeaORMDbErr(err) => write!(f, "{err}"),
            Error::ValidatorValidationErrors(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for Error {}

/// An enum representing the different database instances
#[derive(Debug)]
pub enum DatabaseInstance {
    /// The development instance
    ///
    /// Used for local development and testing. No automated tests are run
    /// against this instance. It is, however, used to generate the entities.
    Development,

    /// The unit test instance
    ///
    /// Used for automated testing. It is truncated and re-seeded before each
    /// test as needed.
    Unit,
}

impl DatabaseInstance {
    /// Gets the name of the database instance as it is known within PostgreSQL.
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
