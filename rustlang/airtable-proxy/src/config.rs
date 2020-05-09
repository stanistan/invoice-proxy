use crate::error::Error;

#[derive(Debug)]
pub(crate) struct Config {
    pub key: String,
    pub base: String,
}

#[derive(Debug)]
pub struct EnvKeys([ &'static str; 2 ]);

impl std::fmt::Display for EnvKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}

impl Config {
    const KEYS: EnvKeys = EnvKeys(["AIRTABLE_KEY", "AIRTABLE_APP"]);

    pub(crate) fn from_env() -> Result<Self, crate::error::Error> {
        match Self::KEYS {
            EnvKeys([key, base]) => match (std::env::var(key), std::env::var(base)) {
                (Ok(key), Ok(base)) => Ok(Self { key, base }),
                _ => Err(Error::MissingEnvConfig {
                    names: Self::KEYS
                }),
            },
        }
    }

    pub(crate) fn table_url(&self, table: &str) -> String {
        format!(
            "https://api.airtable.com/v0/{base}/{table}",
            base = self.base,
            table = table
        )
    }
}
