#[derive(Debug)]
pub(crate) struct Config {
    pub key: String,
    pub base: String,
}

impl Config {
    pub(crate) fn from_env() -> Result<Self, crate::error::Error> {
        use std::env;
        match (env::var("AIRTABLE_KEY"), env::var("AIRTABLE_APP")) {
            (Ok(key), Ok(base)) => Ok(Self { key, base }),
            _ => Err(crate::error::Error::MissingEnvConfig),
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
