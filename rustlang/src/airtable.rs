use crate::error::Error;
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
pub(crate) struct RequestCache {
    hits: u32,
    misses: u32,
    storage: HashMap<Url, Value>,
}

impl RequestCache {
    fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            storage: HashMap::new(),
        }
    }

    pub(crate) fn stats(&self) -> (u32, u32) {
        (self.hits, self.misses)
    }

    pub(crate) fn clear(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.storage.clear();
    }
}

type JSONResult = Result<Value, reqwest::Error>;

impl RequestCache {
    async fn get_or_insert_with<
        G: std::future::Future<Output = JSONResult>,
        F: FnOnce(Url) -> G,
    >(
        &mut self,
        url: Url,
        f: F,
    ) -> JSONResult {
        // we have it!
        if let Some(value) = self.storage.get(&url) {
            self.hits += 1;
            return Ok(value.clone());
        }

        // ok we don't have it.
        self.misses += 1;
        let value = f(url.clone()).await?;
        self.storage.insert(url, value.clone());
        Ok(value)
    }
}

#[derive(Debug)]
pub struct FetchCtx {
    config: Config,
    client: reqwest::Client,
    pub(crate) cache: RequestCache,
}

async fn fetch_url(client: reqwest::Client, url: Url, auth: &str) -> JSONResult {
    client.get(url).bearer_auth(auth).send().await?.json().await
}

impl FetchCtx {
    /// Creates a `FetchCtx` from the environment.
    ///
    /// Required env vars are `AIRTABLE_KEY`, and `AIRTABLE_APP`.
    pub fn from_env() -> Result<Self, &'static str> {
        let config = Config::from_env()?;
        Ok(Self {
            config,
            cache: RequestCache::new(),
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

pub mod request {

    use super::response::{Many, One};
    use super::{Error, FetchCtx, Table};
    use std::marker::PhantomData;

    pub struct QueryParam<'a, 'b, T> {
        pub(super) key: &'a str,
        pub(super) value: &'b str,
        _phantom: PhantomData<T>,
    }

    impl<'a, 'b, T> QueryParam<'a, 'b, T> {
        pub fn new(key: &'a str, value: &'b str) -> Self {
            Self {
                key,
                value,
                _phantom: PhantomData,
            }
        }
    }

    pub async fn query<U: Table>(
        ctx: &mut FetchCtx,
        p: QueryParam<'_, '_, U>,
    ) -> Result<Many<U::Fields>, Error> {
        ctx.fetch_query(U::NAME, &p.key, &p.value).await
    }

    pub async fn fetch_one<T: AsRef<str>, U: Table>(
        ctx: &mut FetchCtx,
        id: T,
    ) -> Result<One<U::Fields>, Error> {
        ctx.fetch_id(U::NAME, id.as_ref()).await
    }

    pub async fn fetch_many<T: AsRef<str>, U: Table>(
        ctx: &mut FetchCtx,
        ids: Vec<T>,
    ) -> Result<Vec<One<U::Fields>>, Error> {
        let mut fetched = Vec::with_capacity(ids.len());
        for id in ids {
            fetched.push(fetch_one::<_, U>(ctx, id).await?);
        }
        Ok(fetched)
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
