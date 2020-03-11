use std::net::SocketAddr;

mod airtable;
mod error;
mod gen_schema;
mod schema;
mod ctx;
mod transform;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    //
    // make sure we can airtable, yo, that we have the
    // right env setup/permissions here.
    let ctx = airtable::FetchCtx::from_env()?;

    //
    // get our server port from the environment, default is 3000
    let address: SocketAddr = std::env::var("HOST")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
        .parse()?;

    //
    // make our context async ready
    let ctx = ctx::wrap_ctx(ctx);

    //
    // grab the generated router
    let router = schema::route(ctx);

    println!("Will be serving the app on {}", address);
    warp::serve(router).run(address).await;

    Ok(())
}
