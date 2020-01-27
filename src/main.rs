mod airtable;
mod gen_schema;
mod schema;
mod transform;

#[tokio::main]
async fn main() -> Result<(), transform::Error> {
    use airtable::FetchCtx;
    use schema::*;

    //
    // We need to have the config in order to be able to talk
    // to the Airtable API at all.
    let ctx = FetchCtx::from_env().unwrap();
    let an_invoice = invoice::one::query(&ctx, "ID", "01").await?;

    //let an_invoice = invoice::one::get(&ctx, "recLYHi5nzYLlHseu").await?;
    dbg!(invoice::one::map(&ctx, an_invoice).await?);

    Ok(())
}
