use crate::airtable;

use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};

/// The context we pass to our routes is
/// wrapped in a mutex so we can safely
/// access it...
///
/// It needs to be in a mutex is because we have
/// a request cache that we read from, write to
/// for this server, which, is obviously mutable.
pub(crate) type Ctx = Arc<Mutex<airtable::FetchCtx>>;

/// This function creates a filter that adds a context
/// to all the endpoints that need it.
pub(crate) fn with_ctx(ctx: Ctx) -> impl Filter<Extract = (Ctx,), Error = Infallible> + Clone {
    warp::any().map(move || ctx.clone())
}

pub mod ctx_cache {
    use super::*;

    /// Shows the stats for the cache of the `FetchCtx`.
    async fn show(ctx: Ctx) -> Result<impl Reply, Rejection> {
        let ctx = ctx.lock().await;
        let (hits, misses) = ctx.cache.stats();
        Ok(warp::reply::json(&json!({
            "hits": hits,
            "misses": misses,
        })))
    }

    /// Clears the cache for the `FetchCtx`.
    async fn clear(ctx: Ctx) -> Result<impl Reply, Rejection> {
        {
            let mut ctx = ctx.lock().await;
            ctx.cache.clear();
        }
        show(ctx).await
    }

    pub fn route(
        ctx: Ctx,
    ) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let show_stats = warp::path!("cache" / "stats")
            .and(warp::get())
            .and(with_ctx(ctx.clone()))
            .and_then(show);

        let clear_stats = warp::path!("cache" / "clear")
            .and(warp::get())
            .and(with_ctx(ctx))
            .and_then(clear);

        show_stats.or(clear_stats)
    }
}

/// Starts serving the configured server at the given port.
pub(crate) async fn start(
    ctx: airtable::FetchCtx,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = Arc::new(Mutex::new(ctx));

    // build routes
    let get_invoice = crate::schema::invoice::endpoints::route(ctx.clone());

    let router = get_invoice.or(ctx_cache::route(ctx));

    warp::serve(router).run(([127, 0, 0, 1], port)).await;
    Ok(())
}
