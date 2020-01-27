use crate::airtable::FetchCtx;

#[derive(Debug)]
pub enum Error {
    Req(reqwest::Error),
    Map(&'static str),
}

pub type MaybeBool = Option<bool>;
pub type IDs = Vec<String>;

pub async fn copy<'a, T: Copy>(_ctx: &'a FetchCtx, t: T) -> Result<T, Error> {
    Ok(t)
}

pub async fn id<'a, T: Sized>(_ctx: &'a FetchCtx, t: T) -> Result<T, Error> {
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
    use num_format::{Locale, WriteFormatted};
    let mut buf = String::from("$");
    if let Err(_) = buf.write_formatted(&val, &Locale::en) {
        return Err(Error::Map("could not format money"));
    }
    buf.push_str(".00");
    Ok(buf)
}

pub async fn split_lines(_ctx: &FetchCtx, val: String) -> Result<Vec<String>, Error> {
    Ok(val.split("\n").map(|s| s.to_owned()).collect())
}
