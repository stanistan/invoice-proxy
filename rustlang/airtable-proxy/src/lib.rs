pub use tokio;
pub use serde;
pub use warp;

pub mod airtable;
pub mod config;
pub mod ctx;
pub mod error;
pub mod gen_schema;
pub mod network;
pub mod transform;

#[macro_export]
macro_rules! execute_schema {
    ($namespace:ident) => {

        #[tokio::main]
        async fn main() -> Result<(), dyn std::error::Error + Send + Sync>> {
            //
            // make sure we can airtable, yo, that we have the
            // right env setup/permissions here.
            let ctx = $crate::airtable::FetchCtx::from_env()?;

            //
            // get our server port from the environment, default is 3000
            let address: SocketAddr = std::env::var("HOST")
                .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
                .parse()?;

            //
            // make our context async ready
            let ctx = $crate::ctx::wrap_ctx(ctx);

            //
            // grab the generated router
            //
            // TODO: this should print/log the routes, which means
            // that `route` methods should probably return a tuple
            // of the `warp::Filter` with some `Debug` struct that we
            // can output here.
            let router = $namespace::gen::route(ctx);

            println!("Will be serving the app on {}", address);
            $crate::warp::serve(router).run(address).await;

            Ok(())
        }

    }
}
