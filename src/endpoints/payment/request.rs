use crate::endpoints::payment::schema::{ClientEnvironment, OrderData};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreatePaymentRequest {
    #[serde(rename = "env")]
    #[builder(setter(into))]
    pub environment: ClientEnvironment,

    /// Merchant unique identifier
    #[builder(setter(into))]
    pub merchant_id: String,

    #[serde(rename = "order")]
    #[builder(setter(into))]
    pub data: OrderData,

    /// The merchant page link that the user is redirected to after completing the payment.
    #[builder(setter(into))]
    pub redirect_url: String,
}
