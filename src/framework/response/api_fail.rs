use crate::framework::response::ApiResultMessage;
use crate::framework::signature::VerifySignatureError;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct EmptyStruct {}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct ApiErrors {
    pub data: ApiErrorData,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct ApiErrorData {
    pub result: ApiResultMessage,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiFailure {
    #[error("HTTP {0} - {1:?}")]
    Error(reqwest::StatusCode, ApiErrors),

    #[error(transparent)]
    Invalid(#[from] reqwest::Error),

    #[error("Decoding Error - {0}")]
    Decoding(#[from] serde_json::Error),

    #[error(transparent)]
    InvalidSignature(#[from] VerifySignatureError),

    #[error("missing required header: {0}")]
    MissingHeader(String),
}

impl PartialEq for ApiFailure {
    fn eq(&self, other: &ApiFailure) -> bool {
        use ApiFailure::*;
        match (self, other) {
            (Error(status1, e1), Error(status2, e2)) => status1 == status2 && e1 == e2,
            (Invalid(e1), Invalid(e2)) => e1.to_string() == e2.to_string(),
            (Decoding(e1), Decoding(e2)) => e1.to_string() == e2.to_string(),
            (InvalidSignature(e1), InvalidSignature(e2)) => e1.to_string() == e2.to_string(),
            (MissingHeader(h1), MissingHeader(h2)) => h1 == h2,
            _ => false,
        }
    }
}
impl Eq for ApiFailure {}
