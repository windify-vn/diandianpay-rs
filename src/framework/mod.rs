pub mod client;
pub mod endpoint;
pub mod response;
pub mod serde;
pub mod signature;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error via the `reqwest` crate
    #[error("Reqwest returned an error when connecting to the Mailgun API: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug)]
pub enum Environment {
    Production,
    Test,
    Custom(String),
}

impl From<&Environment> for url::Url {
    fn from(env: &Environment) -> Self {
        match env {
            Environment::Production => {
                url::Url::parse("https://api.diandianpay.com/api/v1/").unwrap()
            }
            Environment::Test => {
                url::Url::parse("https://test-api.diandianpay.com/api/v1/").unwrap()
            }
            Environment::Custom(url) => url::Url::parse(url.as_str()).unwrap(),
        }
    }
}
