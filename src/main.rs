mod airtable;
mod gen_schema;
mod schema;
mod transform;

use airtable::FetchCtx;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

type Ctx = Arc<Mutex<FetchCtx>>;

fn with_ctx(ctx: Ctx) -> impl Filter<Extract = (Ctx,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || ctx.clone())
}

async fn fetch_invoice_for_id(id: String, ctx: Ctx) -> Result<impl warp::Reply, warp::Rejection> {
    use schema::invoice::one::*;

    let mut fetch_ctx = ctx.lock().await;
    if let Ok(invoice) = query(&mut fetch_ctx, "ID", &id).await {
        if let Ok(invoice) = map(&mut fetch_ctx, invoice).await {
            return Ok(warp::reply::json(&invoice));
        }
    }

    Err(warp::reject::not_found())
}

async fn show_ctx_cache_stats(ctx: Ctx) -> Result<impl warp::Reply, warp::Rejection> {
    let ctx = ctx.lock().await;
    let (hits, misses) = ctx.cache.stats();
    Ok(warp::reply::json(&json!({
        "hits": hits,
        "misses": misses,
    })))
}

async fn clear_ctx_cache(ctx: Ctx) -> Result<impl warp::Reply, warp::Rejection> {
    {
        let mut ctx = ctx.lock().await;
        ctx.cache.clear();
    }
    show_ctx_cache_stats(ctx).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // env based setup
    let server_port: u16 = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()?;

    let ctx = Arc::new(Mutex::new(FetchCtx::from_env()?));

    // build routes
    let get_invoice = warp::path!("invoice" / String)
        .and(warp::get())
        .and(with_ctx(ctx.clone()))
        .and_then(fetch_invoice_for_id);
    let cache_stats = warp::path!("cache" / "stats")
        .and(warp::get())
        .and(with_ctx(ctx.clone()))
        .and_then(show_ctx_cache_stats);
    let cache_clear = warp::path!("cache" / "clear")
        .and(warp::get())
        .and(with_ctx(ctx.clone()))
        .and_then(clear_ctx_cache);
    let router = get_invoice.or(cache_stats).or(cache_clear);

    // serve our business
    println!("Listening on port = {}", server_port);
    warp::serve(router).run(([127, 0, 0, 1], server_port)).await;
    Ok(())
}
