#[derive(Debug)]
pub enum Error {
    Map(&'static str),
    Req(reqwest::Error),
    SerdeTransform(serde_json::error::Error),
    UrlParser(url::ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::Map(ref e) => write!(f, "Mapping error: {}", e),
            Error::Req(ref e) => write!(f, "Reqwest error: {}", e),
            Error::SerdeTransform(ref e) => write!(f, "Deserialization error: {}", e),
            Error::UrlParser(ref e) => write!(f, "Url formatting error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
