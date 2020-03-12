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

pub(crate) fn wrap_ctx(ctx: airtable::FetchCtx) -> Ctx {
    Arc::new(Mutex::new(ctx))
}

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
        let stats = ctx.cache.stats();
        Ok(warp::reply::json(&json!({
            "hits": stats.hits,
            "misses": stats.misses,
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

    pub fn route(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        // GET /cache
        let cache = warp::path("cache").and(warp::get());

        // the endpoints
        let show_stats = warp::path("stats")
            .and(with_ctx(ctx.clone()))
            .and_then(show);
        let clear_stats = warp::path("clear").and(with_ctx(ctx)).and_then(clear);

        cache.and(show_stats.or(clear_stats))
    }
}
