pub use warp;
pub use serde;
pub use tokio;

pub mod airtable;
pub mod config;
pub mod ctx;
pub mod error;
pub mod gen_schema;
pub mod network;
pub mod transform;

#[macro_export]
macro_rules! run_proxy {
    ($namespace:ident) => {
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            $crate::run_proxy!(@inner $namespace)
        }
    };
    (@inner $namespace:ident) => {{
        //
        // make sure we can airtable, yo, that we have the
        // right env setup/permissions here.
        let ctx = $crate::airtable::FetchCtx::from_env()?;

        //
        // get our server port from the environment, default is 3000
        let address: ::std::net::SocketAddr = ::std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
            .parse()?;

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

        //TODO: LOGGER
        println!("will be serving the app on {}", address);
        $crate::warp::serve(router).run(address).await;

        Ok(())
    }};
}
