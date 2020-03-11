use super::response::{Many, One};
use crate::airtable::FetchCtx;
use crate::error::Error;
use crate::gen_schema::Table;
use std::marker::PhantomData;

pub enum Param<T> {
    Query {
        key: String,
        value: String,
        _table: PhantomData<T>,
    },
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

pub async fn one<U: Table>(ctx: &mut FetchCtx, param: Param<U>) -> Result<One<U::Fields>, Error> {
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

pub async fn many<U: Table>(
    ctx: &mut FetchCtx,
    param: Param<U>,
) -> Result<Vec<One<U::Fields>>, Error> {
    Ok(match param {
        Param::Query { key, value, .. } => {
            let result: Many<U::Fields> = ctx.fetch_query(U::NAME, &key, &value).await?;
            result.records
        }
        Param::IDs { ids, .. } => {
            let mut output = Vec::with_capacity(ids.len());
            for id in ids {
                output.push(ctx.fetch_id(U::NAME, &id).await?);
            }
            output
        }
    })
}
