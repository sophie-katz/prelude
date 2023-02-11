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
