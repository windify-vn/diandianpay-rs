mod api_fail;

pub use api_fail::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone, strum_macros::AsRefStr)]
#[serde(untagged, rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum ApiResultCode {
    Succeeded,
    #[default]
    Failed,
    Pending,
    Custom(String),
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[allow(missing_docs)]
pub enum ApiResultStatus {
    #[serde(rename = "S")]
    Successful,
    #[default]
    #[serde(rename = "F")]
    Failed,
}

pub type ApiResult<ResultType> = Result<ResultType, ApiFailure>;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: ApiData<T>,
}

#[derive(Debug, Deserialize)]
pub struct ApiData<T> {
    #[serde(flatten)]
    pub data: T,
    pub result: ApiResultMessage,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct ApiResultMessage {
    #[serde(rename = "result_code")]
    pub code: ApiResultCode,
    #[serde(default, rename = "result_status")]
    pub status: ApiResultStatus,
    #[serde(rename = "result_message")]
    pub message: Option<String>,
}

pub trait JsonResult: DeserializeOwned + Debug {}

pub trait ApiResponseType: Sized {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure>;
}

impl<T> ApiResponseType for T
where
    T: JsonResult,
{
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        serde_json::from_slice(bytes).map_err(ApiFailure::Decoding)
    }
}

impl ApiResponseType for String {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        let text = String::from_utf8_lossy(bytes);

        Ok(text.into_owned())
    }
}

impl ApiResponseType for Vec<u8> {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        Ok(bytes.to_vec())
    }
}

impl ApiResponseType for () {
    fn from_response(_: &bytes::Bytes) -> Result<Self, ApiFailure> {
        Ok(())
    }
}
