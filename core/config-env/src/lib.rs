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

//! Configuration loader for environment variables.

use std::{
    error,
    fmt::{self, Display},
    num::ParseIntError,
};

/// Error type for this crate
#[derive(Debug)]
pub enum Error {
    /// Error for when an environemnt variable has an empty value
    KeyEmpty {
        /// The errneous key
        key: &'static str,
    },
    /// Wrapper for dotenv errors
    DotEnvError(dotenv::Error),
    /// Wrapper for integer parsing errors
    ParseError(ParseIntError),
}

impl From<dotenv::Error> for Error {
    fn from(value: dotenv::Error) -> Self {
        Self::DotEnvError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KeyEmpty { key } => write!(f, "key is empty: {key:#}"),
            Error::DotEnvError(err) => write!(f, "{err}"),
            Error::ParseError(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for Error {}

/// A configuration structure that contains all of the loaded keys from the shell environment and from `.env`.
#[derive(Debug)]
pub struct Configuration {
    /// Loaded from `POSTGRES_HOST`. Hostname or IP address of the server on which PostgreSQL is running.
    pub postgres_host: String,

    /// Loaded from `POSTGRES_PORT`. The port on which PostgreSQL is exposed.
    pub postgres_port: u32,

    /// Loaded from `POSTGRES_USER`. The username to use to login to PostgreSQL.
    pub postgres_user: String,

    /// Loaded from `POSTGRES_PASSWORD`. The password to use to login to PostgreSQL.
    pub postgres_password: String,
}

impl Configuration {
    /// Loads a Configuration from the shell environment and from `.env`.
    ///
    /// # Errors
    ///
    /// Iff there is any issue with dotenv, an error will be returned. Some keys
    /// are for integers; if there is any issue parsing them, an error will be
    /// returned. If an environment variable is empty or undefined, an error will
    /// be thrown.
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv()?;

        Ok(Self {
            postgres_host: Self::get_var_safe("POSTGRES_HOST")?,
            postgres_port: Self::get_var_safe("POSTGRES_PORT")?.parse::<u32>()?,
            postgres_user: Self::get_var_safe("POSTGRES_USER")?,
            postgres_password: Self::get_var_safe("POSTGRES_PASSWORD")?,
        })
    }

    fn get_var_safe(key: &'static str) -> Result<String, Error> {
        let result = dotenv::var(key)?;

        if result.is_empty() {
            return Err(Error::KeyEmpty { key });
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let cfg = Configuration::new().unwrap();

        assert!(!cfg.postgres_host.is_empty());
        assert!(cfg.postgres_port > 0);
        assert!(!cfg.postgres_user.is_empty());
        assert!(!cfg.postgres_password.is_empty());
    }
}
