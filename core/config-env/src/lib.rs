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

use dotenv;
use std::{
    error,
    fmt::{self, Display},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum Error {
    KeyEmpty { key: &'static str },
    DotEnvError(dotenv::Error),
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

#[derive(Debug)]
pub struct Configuration {
    pub postgres_host: String,
    pub postgres_port: u32,
    pub postgres_user: String,
    pub postgres_password: String,
    pub database_url: String,
}

impl Configuration {
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv()?;

        Ok(Self {
            postgres_host: Self::get_var_safe("POSTGRES_HOST")?,
            postgres_port: Self::get_var_safe("POSTGRES_PORT")?.parse::<u32>()?,
            postgres_user: Self::get_var_safe("POSTGRES_USER")?,
            postgres_password: Self::get_var_safe("POSTGRES_PASSWORD")?,
            database_url: Self::get_var_safe("DATABASE_URL")?,
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
        assert!(!cfg.database_url.is_empty());
    }
}
