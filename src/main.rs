mod airtable;
mod gen_schema;
mod schema;
mod transform;

use airtable::FetchCtx;
use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;

type Ctx = Arc<FetchCtx>;

async fn fetch_invoice_for_id(id: String, ctx: Ctx) -> Result<impl warp::Reply, warp::Rejection> {
    use schema::invoice::one::*;


    //
    // i'll want this to be a mutable thing...
    // for caching, and counting stats for requests
    // made maybe???
    // TODO: ^^^
    //
    if let Ok(invoice) = query(&ctx, "ID", &id).await {
        if let Ok(invoice) = map(&ctx, invoice).await {
            return Ok(warp::reply::json(&invoice));
        }
    }


    Err(warp::reject::not_found())
}

fn with_ctx(ctx: Ctx) -> impl Filter<Extract = (Ctx,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || ctx.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let ctx = Arc::new(FetchCtx::from_env()?);

    let invoice = warp::path!("invoice" / String)
        .and(warp::get())
        .and(with_ctx(ctx.clone()))
        .and_then(fetch_invoice_for_id);

    warp::serve(invoice).run(([ 127, 0, 0, 1 ], 3000)).await;








    //
    // We need to have the config in order to be able to talk
    // to the Airtable API at all.
    //let ctx = FetchCtx::from_env().unwrap();
    //let an_invoice = invoice::one::query(&ctx, "ID", "01").await?;

    //let an_invoice = invoice::one::get(&ctx, "recLYHi5nzYLlHseu").await?;
    //dbg!(invoice::one::map(&ctx, an_invoice).await?);

    Ok(())
}
