//!
//! Every public function in this module should be...
//!
//! 1. async
//! 2. take a ctx as a first parameter,
//! 3. take the second parameter via ownership
//! 4. return a `Result<_, Error>` which is defined below.
//!

#![allow(unused)]

use crate::airtable::FetchCtx;
use crate::error::Error;

pub type MaybeBool = Option<bool>;
pub type IDs = Vec<String>;

#[macro_export]
macro_rules! compose {

    ($c:expr, $e:expr, [ ]) => {
        Ok($e)
    };

    ($c:expr, $e:expr, [ $t:expr ]) => {
        $t($c, $e).await
    };

    ($c:expr, $e:expr, [ $t:expr, $($ts:expr),* ]) => {
        match $t($c, $e).await {
            Ok(val) => compose!($c, val, [ $($ts),* ]),
            Err(e) => Err(e)
        }
    };
}

#[macro_export]
macro_rules! pure_fn {
    (
        $name: ident $(<$T:ident $(: $tokens:tt)?>)?
        (
            $arg_name:ident : $arg_type:ty
        )
        -> $ret:ty { $($body:tt)* }
    ) => {
        pub async fn $name $(<$T $(: $tokens)?>)?(_: &FetchCtx, $arg_name: $arg_type) -> Result<$ret, Error> {
            $($body)*
        }
    }
}

pure_fn!(copy<T: Copy>(t: T) -> T {
    Ok(t)
});

pure_fn!(id<T: Sized>(t: T) -> T {
    Ok(t)
});

pure_fn!(force_bool(val: MaybeBool) -> bool {
    Ok(val.unwrap_or(false))
});

pure_fn!(money(val: u32) -> String {
    // FIXME: this doesn't do anything with decimals :(
    use num_format::{Locale, WriteFormatted};
    let mut buf = String::from("$");
    if buf.write_formatted(&val, &Locale::en).is_err() {
        return Err(Error::Map("could not format money"));
    }
    buf.push_str(".00");
    Ok(buf)
});

pure_fn!(split_lines(val: String) -> Vec<String> {
    Ok(val.split('\n').map(|s| s.to_owned()).collect())
});

fn get_first<T>(mut vec: Vec<T>) -> Result<T, Error> {
    match vec.get(0) {
        None => Err(Error::Map("Cannot get the first item from an empty vec")),
        _ => Ok(vec.swap_remove(0)),
    }
}

pure_fn!(first<T>(vec: Vec<T>) -> T {
    get_first(vec)
});

pure_fn!(into_vec<T>(value: T) -> Vec<T> {
    Ok(vec![value])
});
