use reqwest::{Error, Url};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) type JSONResult = Result<Value, Error>;

#[derive(Debug)]
pub(crate) struct Cache {
    hits: u32,
    misses: u32,
    storage: HashMap<Url, Value>,
}

impl Cache {
    pub(crate) fn new() -> Self {
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

    pub(crate) async fn get_or_insert_with<
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

