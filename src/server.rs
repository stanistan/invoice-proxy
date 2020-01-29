use super::{airtable, schema};
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::reject::not_found;
use warp::{Filter, Rejection, Reply};

/// The context we pass to our routes is
/// wrapped in a mutex so we can safely
/// access it...
///
/// It needs to be in a mutex is because we have
/// a request cache that we read from, write to
/// for this server, which, is obviously mutable.
type Ctx = Arc<Mutex<airtable::FetchCtx>>;

/// This function creates a filter that adds a context
/// to all the endpoints that need it.
fn with_ctx(ctx: Ctx) -> impl Filter<Extract = (Ctx,), Error = Infallible> + Clone {
    warp::any().map(move || ctx.clone())
}

/// Fetches an invoice by id.
async fn fetch_invoice_for_id(id: String, ctx: Ctx) -> Result<impl Reply, Rejection> {
    use schema::invoice::one::*;

    let mut fetch_ctx = ctx.lock().await;
    if let Ok(invoice) = query(&mut fetch_ctx, "ID", &id).await {
        if let Ok(invoice) = map(&mut fetch_ctx, invoice).await {
            return Ok(warp::reply::json(&invoice));
        }
    }

    Err(not_found())
}

/// Shows the stats for the cache of the `FetchCtx`.
async fn show_ctx_cache_stats(ctx: Ctx) -> Result<impl Reply, Rejection> {
    let ctx = ctx.lock().await;
    let (hits, misses) = ctx.cache.stats();
    Ok(warp::reply::json(&json!({
        "hits": hits,
        "misses": misses,
    })))
}

/// Clears the cache for the `FetchCtx`.
async fn clear_ctx_cache(ctx: Ctx) -> Result<impl Reply, Rejection> {
    {
        let mut ctx = ctx.lock().await;
        ctx.cache.clear();
    }
    show_ctx_cache_stats(ctx).await
}

/// Starts serving the configured server at the given port.
pub(crate) async fn start(
    ctx: airtable::FetchCtx,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = Arc::new(Mutex::new(ctx));

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

    warp::serve(router).run(([127, 0, 0, 1], port)).await;
    Ok(())
}
