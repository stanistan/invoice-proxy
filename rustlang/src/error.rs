use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    MissingEnvConfig([&'static str; 2]),
    Map(&'static str),
    Req(reqwest::Error),
    SerdeTransform(serde_json::error::Error),
    UrlParser(url::ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        use Error::*;
        match self {
            MissingEnvConfig(vars) => write!(f, "Expected environment vars: {:?}", vars),
            Map(ref e) => write!(f, "Mapping error: {}", e),
            Req(ref e) => write!(f, "Reqwest error: {}", e),
            SerdeTransform(ref e) => write!(f, "Deserialization error: {}", e),
            UrlParser(ref e) => write!(f, "Url formatting error: {}", e),
        }
    }
}

impl warp::reject::Reject for Error {}

impl std::error::Error for Error {}
