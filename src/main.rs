mod airtable;
mod gen_schema;
mod transform;
mod schema;

#[tokio::main]
async fn main() -> Result<(), transform::Error> {

    use schema::*;
    use airtable::FetchCtx;

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
