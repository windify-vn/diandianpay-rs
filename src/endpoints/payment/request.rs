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

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct QueryPaymentRequest {
    /// Merchant unique identifier
    #[builder(setter(into))]
    pub merchant_id: String,

    /// Merchant Order Number
    #[builder(default, setter(strip_option, into))]
    pub merchant_order_id: Option<String>,

    /// Unique identifier of the DianDian system
    #[builder(default, setter(strip_option, into))]
    pub id: Option<String>,
}
