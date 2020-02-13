mod airtable;
mod gen_schema;
mod schema;
mod server;
mod transform;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // make sure we can airtable, yo
    let ctx = airtable::FetchCtx::from_env()?;

    // get our server port from the environment
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()?;

    println!("Will be serving the app on 127.0.0.1:{}", port);
    server::start(ctx, port).await
}
