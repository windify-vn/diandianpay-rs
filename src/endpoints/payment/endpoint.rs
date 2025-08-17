use crate::endpoints::payment::request::CreatePaymentRequest;
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;
use crate::endpoints::payment::response::CreatePaymentResponse;
use crate::framework::response::ApiResponse;

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
