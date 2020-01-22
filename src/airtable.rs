#[derive(Debug)]
pub(crate) struct Config {
    pub key: String,
    pub base: String,
}

impl Config {
    pub(crate) fn from_env() -> Result<Self, &'static str> {
        use std::env;
        match (env::var("AIRTABLE_KEY"), env::var("AIRTABLE_APP")) {
            (Ok(key), Ok(base)) => Ok(Self { key, base }),
            _ => Err("Expected env variables AIRTABLE_KEY, and AIRTABLE_APP to be set"),
        }
    }
}

pub struct FetchCtx {
    config: Config,
    client: reqwest::Client,
}

impl FetchCtx {
    /// Creates a `FetchCtx` from the environment.
    ///
    /// Required env vars are `AIRTABLE_KEY`, and `AIRTABLE_APP`.
    pub fn from_env() -> Result<Self, &'static str> {
        let config = Config::from_env()?;
        Ok(Self {
            config,
            client: reqwest::Client::new(),
        })
    }

    pub fn id_request(&self, table: &str, id: &str) -> reqwest::RequestBuilder {
        let url = format!(
            "https://api.airtable.com/v0/{base}/{table}/{id}",
            base = self.config.base,
            table = table,
            id = id
            );
        self.client.get(&url).bearer_auth(&self.config.key)
    }
}

pub mod response {

    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct One<T> {
        pub id: String,
        pub fields: T,
        #[serde(rename = "createdTime")]
        pub created_time: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Many<T> {
        pub records: Vec<One<T>>,
    }
}
