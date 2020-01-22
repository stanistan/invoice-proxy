mod airtable;
mod gen_schema;
mod schema;

pub fn copy<T: Copy>(t: &T) -> T {
    *t
}

pub fn id<T: ?Sized>(t: &T) -> &T {
    t
}

pub fn first<'a, T>(list: &'a Vec<T>) -> &'a T {
    list.first().unwrap()
}

pub fn force_bool(val: &Option<bool>) -> bool {
    val.unwrap_or(false)
}

type MaybeBool = Option<bool>;
type IDs = Vec<String>;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    use schema::*;
    use airtable::FetchCtx;

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

    dbg!(invoice::map(&an_invoice).await);

    Ok(())
}
