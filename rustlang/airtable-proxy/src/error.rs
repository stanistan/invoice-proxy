use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error creating record {table}")]
    Create {
        table: &'static str,
        #[source] source: Box<Error>,
    },
    #[error("Missing required environment variables: {names}")]
    MissingEnvConfig{
        names: crate::config::EnvKeys,
    },
    #[error(transparent)]
    Req(reqwest::Error),
    #[error("{message} for table={table}")]
    RequestParams {
        table: &'static str,
        message: &'static str,
    },
    #[error("Recieved a response with status={status} for url={url}")]
    Response { status: String, url: String },
    #[error(transparent)]
    SerdeTransform(serde_json::error::Error),
    #[error("Error during transform function, {message}")]
    Transform {
        message: &'static str,
    },
    #[error(transparent)]
    UrlParser(url::ParseError),
}

impl warp::reject::Reject for Error { }
