//!
//! Every public function in this module should be...
//! 1. async
//! 2. take a ctx as a first parameter,
//! 3. take the second parameter via ownership
//! 4. return a `Result<_, Error>` which is defined below.

use crate::airtable::response;
use crate::airtable::FetchCtx;

#[macro_export]
macro_rules! compose {

    ($c:expr, $e:expr, [ ]) => {
        Ok($e)
    };

    ($c:expr, $e:expr, [ $t:expr ]) => {
        $t($c, $e).await
    };

    ($c:expr, $e:expr, [ $t:expr, $($ts:expr,)* ]) => {
        match $t($c, $e).await {
            Ok(val) => compose!($c, val, [ $($ts,)* ]),
            Err(e) => Err(e)
        }
    };
}

#[derive(Debug)]
pub enum Error {
    Map(&'static str),
    Req(reqwest::Error),
    SerdeTransform(serde_json::error::Error),
    UrlParser(url::ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::Map(ref e) => write!(f, "Mapping error: {}", e),
            Error::Req(ref e) => write!(f, "Reqwest error: {}", e),
            Error::SerdeTransform(ref e) => write!(f, "Deserialization error: {}", e),
            Error::UrlParser(ref e) => write!(f, "Url formatting error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

pub type MaybeBool = Option<bool>;
pub type IDs = Vec<String>;

pub async fn copy<T: Copy>(_ctx: &FetchCtx, t: T) -> Result<T, Error> {
    Ok(t)
}

pub async fn id<T: Sized>(_ctx: &FetchCtx, t: T) -> Result<T, Error> {
    Ok(t)
}

pub async fn first<T>(_ctx: &FetchCtx, mut vec: Vec<T>) -> Result<T, Error> {
    match vec.get(0) {
        None => Err(Error::Map("Cannot get the first item from an empty vec")),
        _ => Ok(vec.swap_remove(0)),
    }
}

pub async fn force_bool(_ctx: &FetchCtx, val: Option<bool>) -> Result<bool, Error> {
    Ok(val.unwrap_or(false))
}

pub async fn money(_ctx: &FetchCtx, val: u32) -> Result<String, Error> {
    // FIXME: this doesn't do anything with decimals :(
    use num_format::{Locale, WriteFormatted};
    let mut buf = String::from("$");
    if buf.write_formatted(&val, &Locale::en).is_err() {
        return Err(Error::Map("could not format money"));
    }
    buf.push_str(".00");
    Ok(buf)
}

pub async fn split_lines(_ctx: &FetchCtx, val: String) -> Result<Vec<String>, Error> {
    Ok(val.split('\n').map(|s| s.to_owned()).collect())
}

pub async fn into_records<T>(
    _ctx: &FetchCtx,
    many: response::Many<T>,
) -> Result<Vec<response::One<T>>, Error> {
    Ok(many.records)
}
