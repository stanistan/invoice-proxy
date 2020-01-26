mod airtable;
mod gen_schema;
mod schema;

use airtable::FetchCtx;

#[derive(Debug)]
pub enum Error {
    Req(reqwest::Error),
    Map(&'static str),
}

pub async fn copy<'a, T: Copy>(_ctx: &'a FetchCtx, t: T) -> Result<T, Error> {
    Ok(t)
}

pub async fn id<'a, T: Sized>(_ctx: &'a FetchCtx, t: T) -> Result<T, Error> {
    Ok(t)
}

pub async fn first<T>(_ctx: &FetchCtx, mut vec: Vec<T>) -> Result<T, Error> {
    match vec.get(0) {
        None => Err(Error::Map("Cannot get the first item from an empty vec")),
        _ => Ok(vec.swap_remove(0))
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

pub async fn get_rate_unit_name(_ctx: &FetchCtx, unit: schema::invoice_rate_unit::Mapped) -> Result<String, Error> {
    Ok(unit.name)
}

type MaybeBool = Option<bool>;
type IDs = Vec<String>;


#[tokio::main]
async fn main() -> Result<(), Error> {

    use schema::*;

    //
    // We need to have the config in order to be able to talk
    // to the Airtable API at all.
    let ctx = FetchCtx::from_env().unwrap();
    let an_invoice = invoice::get_one(&ctx, "recLYHi5nzYLlHseu").await?;

    /*
    let invoice_item_id = dbg!(an_invoice.fields.invoice_items.first().unwrap());
    let client_id = dbg!(an_invoice.fields.client.first().unwrap());

    dbg!(invoice_item::get_one(&ctx, invoice_item_id).await?);
    dbg!(invoice_client::get_one(&ctx, client_id).await?);
    */

    dbg!(invoice::map_one(&ctx, an_invoice).await?);

    Ok(())
}
