#![feature(concat_idents)]

mod airtable;
mod gen_schema;
mod schema;
mod transform;

use airtable::FetchCtx;
use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;

type Ctx = Arc<Mutex<FetchCtx>>;

async fn fetch_invoice_for_id(id: String, ctx: Ctx) -> Result<impl warp::Reply, warp::Rejection> {
    use schema::invoice::one::*;

    let mut fetch_ctx = ctx.lock().await;
    if let Ok(invoice) = query(&mut fetch_ctx, "ID", &id).await {
        if let Ok(invoice) = map(&mut fetch_ctx, invoice).await {
            dbg!(&fetch_ctx);
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

    let ctx = Arc::new(Mutex::new(FetchCtx::from_env()?));

    let invoice = warp::path!("invoice" / String)
        .and(warp::get())
        .and(with_ctx(ctx.clone()))
        .and_then(fetch_invoice_for_id);

    warp::serve(invoice).run(([ 127, 0, 0, 1 ], 3000)).await;
    Ok(())
}
