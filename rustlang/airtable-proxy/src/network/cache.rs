use crate::error::Error;
use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;

pub(crate) type JSONResult = Result<Value, Error>;

#[derive(Debug)]
pub(crate) struct Stats {
    pub hits: u32,
    pub misses: u32,
}

#[derive(Debug)]
pub(crate) struct Cache {
    stats: Stats,
    storage: HashMap<Url, Value>,
}

impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            stats: Stats { hits: 0, misses: 0 },
            storage: HashMap::new(),
        }
    }

    pub(crate) fn stats(&self) -> &Stats {
        &self.stats
    }

    pub(crate) fn clear(&mut self) {
        self.stats.hits = 0;
        self.stats.misses = 0;
        self.storage.clear();
    }

    pub(crate) async fn get_or_insert_with<G: Future<Output = JSONResult>, F: FnOnce(Url) -> G>(
        &mut self,
        url: Url,
        f: F,
    ) -> JSONResult {
        Ok(if let Some(value) = self.storage.get(&url) {
            crate::debug!("hit | url={}", url);
            self.stats.hits += 1;
            value.clone()
        } else {
            crate::debug!("miss | url={}", url);
            self.stats.misses += 1;
            let value = f(url.clone()).await?;
            self.storage.insert(url, value.clone());
            value
        })
    }
}
