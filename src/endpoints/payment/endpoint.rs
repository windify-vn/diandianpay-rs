use crate::endpoints::payment::request::{CreatePaymentRequest, QueryPaymentRequest};
use crate::endpoints::payment::response::{CreatePaymentResponse, QueryPaymentResponse};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use crate::framework::response::ApiResponse;
use http::Method;

impl EndpointSpec for CreatePaymentRequest {
    type ResponseType = ApiResponse<CreatePaymentResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "payment".into()
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(serde_json::to_string(self).unwrap()))
    }
}

impl EndpointSpec for QueryPaymentRequest {
    type ResponseType = ApiResponse<QueryPaymentResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "payment/result".into()
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(serde_json::to_string(self).unwrap()))
    }
}
