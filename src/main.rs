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

pub async fn money<T: std::fmt::Display>(_ctx: &FetchCtx, val: T) -> Result<String,Error> {
    // FIXME lol
    Ok(format!("MONEY FiXME {}", val))
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
    let an_invoice = dbg!(invoice::get_one(&ctx, "recLYHi5nzYLlHseu").await?);

    /*
    let invoice_item_id = dbg!(an_invoice.fields.invoice_items.first().unwrap());
    let client_id = dbg!(an_invoice.fields.client.first().unwrap());

    dbg!(invoice_item::get_one(&ctx, invoice_item_id).await?);
    dbg!(invoice_client::get_one(&ctx, client_id).await?);
    */

    dbg!(invoice::map(&ctx, an_invoice).await?);

    Ok(())
}
