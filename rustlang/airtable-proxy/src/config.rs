use crate::error::Error;

#[derive(Debug)]
pub(crate) struct Config {
    pub key: String,
    pub base: String,
}

impl Config {
    const KEYS: [&'static str; 2] = ["AIRTABLE_KEY", "AIRTABLE_APP"];

    pub(crate) fn from_env() -> Result<Self, crate::error::Error> {
        match Self::KEYS {
            [key, base] => match (std::env::var(key), std::env::var(base)) {
                (Ok(key), Ok(base)) => Ok(Self { key, base }),
                _ => Err(Error::MissingEnvConfig(Self::KEYS)),
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
