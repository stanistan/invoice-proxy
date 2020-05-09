use std::fmt::{Display, Formatter};

pub enum Error {
    Create(&'static str, Box<Error>),
    MissingEnvConfig([&'static str; 2]),
    Map(&'static str),
    Req(reqwest::Error),
    Response { status: String, url: String },
    SerdeTransform(serde_json::error::Error),
    UrlParser(url::ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        use Error::*;
        match self {
            Create(ref name, ref cause) => {
                write!(f, "Could not create {}, caused by={}", name, cause)
            }
            MissingEnvConfig(vars) => write!(f, "Expected environment vars: {:?}", vars),
            Map(ref e) => write!(f, "Mapping error: {}", e),
            Req(ref e) => write!(f, "Reqwest error: {}", e),
            Response {
                ref status,
                ref url,
            } => write!(f, "url={}, failed with status={}", url, status),
            SerdeTransform(ref e) => write!(f, "Deserialization error: {}", e),
            UrlParser(ref e) => write!(f, "Url formatting error: {}", e),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        use Error::*;
        match self {
            Create(ref name, ref cause) => {
                write!(f, "Could not create {}, caused by={}", name, cause)
            }
            MissingEnvConfig(vars) => write!(f, "Expected environment vars: {:?}", vars),
            Map(ref e) => write!(f, "Mapping error: {}", e),
            Req(ref e) => write!(f, "Reqwest error: {}", e),
            Response {
                ref status,
                ref url,
            } => write!(f, "url={}, failed with status={}", url, status),
            SerdeTransform(ref e) => write!(f, "Deserialization error: {}", e),
            UrlParser(ref e) => write!(f, "Url formatting error: {}", e),
        }
    }
}

impl warp::reject::Reject for Error {}

impl std::error::Error for Error {}
