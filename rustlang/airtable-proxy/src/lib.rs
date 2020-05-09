pub use warp;
pub use serde;
pub use tokio;

pub use pretty_env_logger;
pub use log::{debug, info, trace};

pub mod airtable;
pub mod config;
pub mod ctx;
pub mod error;
pub mod gen_schema;
pub mod network;
pub mod transform;

#[macro_export]
macro_rules! start_proxy {
    ($namespace:ident) => {
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            pretty_env_logger::init();
            $crate::start_proxy!(@inner $namespace)
        }
    };
    (@inner $namespace:ident) => {{
        info!("Attempting to start the proxy.");
        //
        // make sure we can airtable, yo, that we have the
        // right env setup/permissions here.
        let ctx = $crate::airtable::FetchCtx::from_env()?;
        debug!("FetchCtx constructed.");

        //
        // get our server port from the environment, default is 3000
        let address: ::std::net::SocketAddr = ::std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
            .parse()?;
        debug!("parsed out address={}", address);

        //
        // make our context async ready
        let ctx = $crate::ctx::wrap_ctx(ctx);

        //
        // grab the generated router
        //
        // todo: this should print/log the routes, which means
        // that `route` methods should probably return a tuple
        // of the `warp::filter` with some `debug` struct that we
        // can output here.
        let router = $namespace::gen::route(ctx);

        info!("Starting proxy on port={}", address);
        $crate::warp::serve(router).run(address).await;

        Ok(())
    }};
}
