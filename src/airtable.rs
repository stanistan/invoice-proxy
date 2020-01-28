use super::transform::Error;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

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

#[derive(Debug)]
pub struct FetchCtx {
    config: Config,
    client: reqwest::Client,
    cache_hits: u32,
    cache_misses: u32,
    cache: HashMap<Url, serde_json::Value>,
}

impl FetchCtx {
    /// Creates a `FetchCtx` from the environment.
    ///
    /// Required env vars are `AIRTABLE_KEY`, and `AIRTABLE_APP`.
    pub fn from_env() -> Result<Self, &'static str> {
        let config = Config::from_env()?;
        Ok(Self {
            config,
            cache_hits: 0,
            cache_misses: 0,
            client: reqwest::Client::new(),
            cache: HashMap::new(),
        })
    }

    fn base_url(&self, table: &str) -> String {
        format!(
            "https://api.airtable.com/v0/{base}/{table}",
            base = self.config.base,
            table = table
        )
    }

    fn id_request(&mut self, table: &str, id: &str) -> reqwest::Url {
        let url = format!("{}/{}", self.base_url(table), id);
        reqwest::Url::parse(&url).unwrap()
    }

    fn query_request(&mut self, table: &str, field: &str, value: &str) -> reqwest::Url {
        let query = format!("{{{field}}} = '{value}'", field = field, value = value);
        reqwest::Url::parse_with_params(&self.base_url(table), &[("filterByFormula", &query)])
            .unwrap()
    }

    async fn fetch<T: DeserializeOwned>(&mut self, url: reqwest::Url) -> Result<T, reqwest::Error> {
        let value: Value;
        if !self.cache.contains_key(&url) {
            self.cache_misses += 1;
            value = self
                .client
                .get(url.clone())
                .bearer_auth(&self.config.key)
                .send()
                .await?
                .json::<Value>()
                .await?;
            self.cache.insert(url, value.clone());
        } else {
            self.cache_hits += 1;
            value = self.cache.get(&url).cloned().unwrap();
        }

        Ok(serde_json::from_value(value).unwrap())
    }

    pub async fn fetch_id<T: DeserializeOwned>(
        &mut self,
        table: &str,
        id: &str,
    ) -> Result<T, Error> {
        let url = self.id_request(table, id);
        self.fetch(url).await.map_err(|e| Error::Req(e))
    }

    pub async fn fetch_query<T: DeserializeOwned>(
        &mut self,
        table: &str,
        field: &str,
        value: &str,
    ) -> Result<T, Error> {
        let url = self.query_request(table, field, value);
        self.fetch(url).await.map_err(|e| Error::Req(e))
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
