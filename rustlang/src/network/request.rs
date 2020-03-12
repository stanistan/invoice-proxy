//!
//! This describes the shapes of requests and how we can make them, generically
//! based on the structure of an airtable `Table`.
//!
//! We can then either do an HTTP Request for `one` or `many` of these `Param`.
//!

use super::response::{List, Many, One};
use crate::airtable::FetchCtx;
use crate::error::Error;
use crate::gen_schema::Table;
use std::marker::PhantomData;

type Result<T> = std::result::Result<T, Error>;

/// The enum represents the kinds of GET requests we can make to the Airtable API.
pub enum Param<T> {
    /// This is a GET Request that will return a list, that we can do a key/value filter over.
    Query {
        key: String,
        value: String,
        _table: PhantomData<T>,
    },
    /// An ID(s) record lookup.
    IDs {
        ids: Vec<String>,
        _table: PhantomData<T>,
    },
}

impl<T: Table> Param<T> {
    pub fn new_query(key: String, value: String) -> Self {
        Param::Query {
            key,
            value,
            _table: PhantomData,
        }
    }

    pub fn new_id(ids: Vec<String>) -> Self {
        Param::IDs {
            ids,
            _table: PhantomData,
        }
    }
}

/// Fetch one item based on the `param` and `ctx`.
pub async fn one<U: Table>(ctx: &mut FetchCtx, param: Param<U>) -> Result<One<U::Fields>> {
    match param {
        Param::Query { key, value, .. } => {
            let result: Many<U::Fields> = ctx.fetch_query(U::NAME, &key, &value).await?;
            crate::transform::first(ctx, result.records).await
        }
        Param::IDs { ids, .. } => {
            if let Some(id) = ids.first() {
                ctx.fetch_id(U::NAME, &id).await
            } else {
                Err(Error::Map("missing ids at param construction"))
            }
        }
    }
}

/// Fetch many items based on the `param` and `ctx`.
pub async fn many<U: Table>(ctx: &mut FetchCtx, param: Param<U>) -> Result<List<U::Fields>> {
    Ok(match param {
        Param::Query { key, value, .. } => {
            let result: Many<U::Fields> = ctx.fetch_query(U::NAME, &key, &value).await?;
            result.records
        }
        Param::IDs { ids, .. } => {
            // TODO: this should not block/await on each loop,
            // but let them all run in parallel until they're done,
            // then we can accumulate them into the output vector.
            let mut output = Vec::with_capacity(ids.len());
            for id in ids {
                output.push(ctx.fetch_id(U::NAME, &id).await?);
            }
            output
        }
    })
}
