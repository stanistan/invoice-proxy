use crate::error::Error;
use reqwest::Url;
use serde::de::DeserializeOwned;
use crate::network::cache::{JSONResult, Cache};

#[derive(Debug)]
pub struct FetchCtx {
    config: crate::config::Config,
    client: reqwest::Client,
    pub(crate) cache: Cache,
}

async fn fetch_url(client: reqwest::Client, url: Url, auth: &str) -> JSONResult {
    client.get(url).bearer_auth(auth).send().await?.json().await
}

impl FetchCtx {
    /// Creates a `FetchCtx` from the environment.
    ///
    /// Required env vars are `AIRTABLE_KEY`, and `AIRTABLE_APP`.
    pub fn from_env() -> Result<Self, &'static str> {
        let config = crate::config::Config::from_env()?;
        Ok(Self {
            config,
            cache: Cache::new(),
            client: reqwest::Client::new(),
        })
    }

    fn base_url(&self, table: &str) -> String {
        format!(
            "https://api.airtable.com/v0/{base}/{table}",
            base = self.config.base,
            table = table
        )
    }

    fn id_request(&mut self, table: &str, id: &str) -> Result<Url, Error> {
        let url = format!("{}/{}", self.base_url(table), id);
        Url::parse(&url).map_err(Error::UrlParser)
    }

    fn query_request(&mut self, table: &str, field: &str, value: &str) -> Result<Url, Error> {
        let query = format!("{{{field}}} = '{value}'", field = field, value = value);
        Url::parse_with_params(&self.base_url(table), &[("filterByFormula", &query)])
            .map_err(Error::UrlParser)
    }

    async fn fetch<T: DeserializeOwned>(&mut self, url: Url) -> Result<T, Error> {
        let client = self.client.clone();
        let key = &self.config.key;
        let value = self
            .cache
            .get_or_insert_with(url, move |u| fetch_url(client, u, key))
            .await
            .map_err(Error::Req)?;
        serde_json::from_value(value).map_err(Error::SerdeTransform)
    }

    pub async fn fetch_id<T: DeserializeOwned>(
        &mut self,
        table: &str,
        id: &str,
    ) -> Result<T, Error> {
        let url = self.id_request(table, id)?;
        self.fetch(url).await
    }

    pub async fn fetch_query<T: DeserializeOwned>(
        &mut self,
        table: &str,
        field: &str,
        value: &str,
    ) -> Result<T, Error> {
        let url = self.query_request(table, field, value)?;
        self.fetch(url).await
    }
}

pub trait Table {
    const NAME: &'static str;
    type Fields: DeserializeOwned;
}
