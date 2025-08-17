use crate::endpoints::payment::AmountUnitFloatError;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy, strum_macros::AsRefStr)]
#[serde(rename_all = "UPPERCASE")]
#[allow(missing_docs)]
pub enum PaymentType {
    Card,
    Paypal,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy, strum_macros::AsRefStr)]
#[allow(missing_docs)]
pub enum DeliveryMethodType {
    #[serde(rename = "Pharmaceutical")]
    Physical,
    #[serde(rename = "DIGITAL")]
    Digital,
}

#[derive(
    Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone, Copy, strum_macros::AsRefStr,
)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
#[allow(missing_docs)]
pub enum TerminalType {
    #[default]
    Web,
    Wap,
    App,
    MiniApp,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, strum_macros::AsRefStr)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum PaymentStatus {
    Succeeded,
    Pending,
    Refunded,
    Failed,
    #[serde(untagged)]
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextAction {
    /// Redirect the customer to this URL to complete
    /// further actions (such as payment or providing additional information)
    pub redirect_url: String,

    /// The URL to redirect the customer back to the merchant
    /// page after completing or canceling the action
    pub return_url: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ClientEnvironment {
    /// Browser environment information for placing an order
    #[builder(setter(into))]
    pub browser_info: BrowserInfo,

    /// Customer IP address
    #[builder(setter(into))]
    pub client_ip: String,

    #[builder(default, setter(strip_option, into))]
    pub device_info: Option<DeviceInfo>,

    /// The terminal types applicable to merchant services.
    /// The valid values are:
    ///
    /// WEB: The client terminal type is a website, which can be opened through a PC browser.
    /// WAP: The client terminal type is H5 page, which can be opened through a mobile browser.
    /// APP: The client terminal type is a mobile application.
    /// MINI-APP: The terminal type on the merchant end is a mobile mini program.
    #[builder(default, setter(into))]
    pub terminal_type: TerminalType,
}

/// Browser environment information for placing an order
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BrowserInfo {
    /// Request header information of user browser
    #[builder(default, setter(strip_option, into))]
    pub accept_header: Option<String>,

    /// Does the user's browser support running Java
    #[builder(default, setter(strip_option, into))]
    pub java_enabled: Option<bool>,

    /// Does the user's browser support running JavaScript
    #[builder(default, setter(strip_option, into))]
    pub java_script_enabled: Option<bool>,

    /// The language of the user's browser.
    /// This value is obtained through the navigator. language attribute of the browser (defined
    /// according to IETF BCP 47)
    #[builder(default, setter(strip_option, into))]
    pub language: Option<String>,

    /// User's browser user agent
    #[builder(setter(into))]
    pub user_agent: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DeviceInfo {
    /// The color depth of the user's browser, expressed in bits per pixel.
    /// The value is obtained through the screen.comlorDepth property of the browser.
    /// The valid values are 1, 4, 8, 15, 16, 24, 30, 32, or 48.
    /// For example, 8 represents 8-bit color depth.
    #[builder(default, setter(strip_option, into))]
    pub color_depth: Option<u32>,

    /// The language of the device ordered by the user
    #[builder(default, setter(strip_option, into))]
    pub device_language: Option<String>,

    /// Token identification of the device
    #[builder(default, setter(strip_option, into))]
    pub device_token_id: Option<String>,

    /// The screen height of the user device (in pixels).
    #[builder(default, setter(strip_option, into))]
    pub screen_height: Option<u32>,

    /// The screen width of the user device (in pixels).
    #[builder(default, setter(strip_option, into))]
    pub screen_width: Option<u32>,

    /// The time difference between UTC time and the user's browser local time, measured in
    /// minutes.
    /// This value is obtained by using the getTimezoneOffset() property.
    /// For example, if the local time of the user's browser is UTC+2, the value of this
    /// parameter is -120.
    #[builder(default, setter(strip_option, into))]
    pub time_zone_offset: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct OrderData {
    /// A unique order ID generated by the merchant, used for transaction tracking and customer
    /// service.
    #[serde(rename = "merchant_order_id")]
    #[builder(setter(into))]
    pub order_id: String,

    /// Product information, including the ID, name, price, and quantity of the products in the
    /// order.
    #[builder(default, setter(into))]
    pub goods: Option<Vec<ItemLine>>,

    /// Customize source data to be represented as key/value
    #[builder(default, setter(strip_option, into))]
    pub metadata: Option<Metadata>,

    /// Payment amount
    #[serde(rename = "payment_amount")]
    #[builder(setter(into))]
    pub amount: Amount,

    /// The payment method used by merchants or acquiring institutions for receiving payments.
    #[serde(rename = "payment_method")]
    #[builder(setter(into))]
    pub method: PaymentMethod,

    /// Delivery information, including recipient (name, phone number, email, and delivery
    /// address) information, as well as delivery service provider information.
    #[builder(default, setter(strip_option, into))]
    pub shipping: Option<ShippingDetails>,
}

/// Up to 100 can be included
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ItemLine {
    /// The delivery method of the product.
    /// The valid values are:
    ///
    /// Pharmaceutical: indicates that the delivery method is physical delivery.
    /// DIGITAL: Indicates that the delivery method is digital delivery.
    #[builder(setter(into))]
    pub delivery_method_type: DeliveryMethodType,

    /// The category of the product.
    /// If the product has multiple levels of classification
    #[serde(rename = "goods_category")]
    #[builder(default, setter(strip_option, into))]
    pub category: Option<String>,

    /// Identify the exclusive ID of the product
    #[serde(rename = "goods_id")]
    #[builder(setter(into))]
    pub id: String,

    /// Product image link
    #[serde(rename = "goods_img_url")]
    #[builder(default, setter(strip_option, into))]
    pub image: Option<String>,

    /// Goods Name
    #[serde(rename = "goods_name")]
    #[builder(setter(into))]
    pub title: String,

    /// The amount value expressed in the form of a positive integer in the smallest currency
    /// unit.
    /// For example, if the currency is USD and the amount is $1.00, set the value of this
    /// parameter to 100.
    /// Alternatively, if the currency is JPY and the amount is ¥ 1, set the value of this
    /// parameter to 1.
    #[serde(rename = "goods_price")]
    #[builder(setter(into))]
    pub price: AmountUnit,

    /// Goods Count
    #[serde(rename = "goods_quantity")]
    #[builder(setter(into))]
    pub quantity: u64,

    /// The website link where the user placed the order
    #[serde(rename = "url")]
    #[builder(setter(into))]
    pub url: String,
}

/// Customize source data to be represented as key/value
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
pub struct Metadata {
    #[builder(default, setter(strip_option, into))]
    pub domain: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub shop: Option<String>,
}

/// The payment method used by merchants or acquiring institutions for receiving payments.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaymentMethod {
    #[builder(default, setter(strip_option, into))]
    #[serde(rename = "payment_data")]
    pub data: Option<PaymentData>,

    /// The payment method types included in the payment method options.
    #[builder(setter(into))]
    pub payment_type: PaymentType,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaymentData {
    #[builder(default, setter(strip_option, into))]
    #[serde(rename = "billing_address")]
    pub address: Option<Address>,

    #[builder(default, setter(strip_option, into))]
    #[serde(rename = "card_holder_name")]
    pub name: Option<PersonName>,

    /// Bank card number
    #[builder(setter(into))]
    pub card_number: String,

    /// Two letter country or region code.
    /// For more information, please refer to the ISO 3166 national code standard.
    #[builder(setter(into))]
    pub country: String,

    /// Card Verification Code (CVV), also known as Card Security Code (CSC) or Card Verification
    /// Code (CVC).
    #[builder(setter(into))]
    pub cvv: String,

    /// The expiration month of the bank card.
    /// Pass in two digits representing the month.
    /// For example, if the expiration month is February, the value of this parameter is 02
    #[builder(setter(into))]
    pub expiry_month: String,

    /// The expiration year of the bank card.
    /// Enter the last two digits of the year.
    /// For example, if the expiration year is 2025, the value of this parameter is 25
    #[builder(setter(into))]
    pub expiry_year: String,

    /// Do you want to enable 3ds verification
    #[builder(setter(strip_bool))]
    pub requires_3ds: bool,
}

/// Delivery information, including recipient (name, phone number, email, and delivery
/// address) information, as well as delivery service provider information.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ShippingDetails {
    /// Delivery service providers for physical products, such as FedEx, UPS, or USPS.
    #[builder(default, setter(strip_option, into))]
    pub carrier: Option<String>,

    /// Customer email
    #[serde(default)]
    #[builder(setter(into))]
    pub email: String,

    /// Customer's mobile phone number
    #[serde(default)]
    #[builder(setter(into))]
    pub phone: String,

    #[serde(default, rename = "shipping_address")]
    #[builder(setter(into))]
    pub address: Address,

    #[serde(default, rename = "shipping_name")]
    /// The name of the payee.
    #[builder(setter(into))]
    pub name: PersonName,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
pub struct Address {
    /// Address line 1, such as street address, post office box, and company name.
    #[serde(default)]
    #[builder(setter(into))]
    pub address1: String,

    /// Address line 2, such as apartment, suite, unit, and building information.
    #[builder(default, setter(strip_option, into))]
    pub address2: Option<String>,

    /// City, district, suburb, town or village name.
    #[serde(default)]
    #[builder(setter(into))]
    pub city: String,

    /// Two letter country or region code.
    /// For more information, please refer to the ISO 3166 national code standard.
    #[serde(default)]
    #[builder(setter(into))]
    pub country: String,

    /// For credit card payments, if your business entity is located in the United States and
    /// your bank card is issued in Canada, the United States, or the United Kingdom, please set
    /// a region code that follows the ISO 3611-2 standard and contains two to three characters.
    #[serde(default)]
    #[builder(setter(into))]
    pub state: String,

    /// Postal code or ZIP code.
    #[serde(default)]
    #[builder(setter(into))]
    pub zip_code: String,
}

/// The name of the payee.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, TypedBuilder)]
pub struct PersonName {
    /// first name
    #[serde(default)]
    #[builder(setter(into))]
    pub first_name: String,

    /// surname
    #[serde(default)]
    #[builder(setter(into))]
    pub last_name: String,

    /// full name
    #[serde(default)]
    #[builder(setter(into))]
    pub full_name: String,
}

/// Payment amount
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Amount {
    /// Currency code, the value is a three digit currency code that follows the ISO 4217
    /// standard.
    #[builder(setter(into))]
    pub currency: String,

    /// The amount value expressed in the form of a positive integer in the smallest currency
    /// unit.
    /// For example, if the currency is USD and the amount is $1.00, set the value of this
    /// parameter to 100
    #[builder(setter(into))]
    pub value: AmountUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentDataSummary {
    Paypal(PaymentPaypalSummary),
    #[serde(untagged)]
    Card(PaymentCardSummary),
    // #[serde(untagged)]
    // Unknown(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodSummary {
    #[serde(rename = "payment_data")]
    pub data: PaymentDataSummary,

    /// The payment method types included in the payment method options.
    pub payment_type: PaymentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    /// Unique identifier of the DianDian system
    pub id: String,

    /// The unique ID for identifying orders is assigned by the merchants who directly provide
    /// services or goods to customers. This field is used for displaying users' consumption
    /// records as well as for other further operations such as dispute tracking or handling
    /// customer complaints.
    pub merchant_order_id: String,

    /// The payment method used by merchants or acquiring institutions for receiving payments.
    pub payment_method: PaymentMethodSummary,

    /// Payment status
    pub payment_status: PaymentStatus,

    /// The proportion of acquiring merchants
    #[serde(rename = "payment_amount")]
    pub gross_amount: Amount,

    /// Total refund amount
    pub refunded_amount: Option<AmountUnit>,

    /// Transaction fee
    pub transaction_fee: Option<AmountUnit>,

    /// Delivery information, including the recipient's details (name, phone number, email
    /// address and delivery address), as well as information about the delivery service provider.
    pub shipping: Option<ShippingDetails>,

    /// Is there any dispute regarding the order
    #[serde(default)]
    pub is_dispute: bool,

    /// Reason for failure
    pub failure_reason: Option<String>,

    /// Custom source data (in the form of key/value pairs)
    #[serde(default)]
    pub metadata: Metadata,

    /// The latest update time of the order
    pub update_at: chrono::DateTime<chrono::Utc>,

    /// Creation Date
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<(&str, AmountUnit)> for Amount {
    fn from(value: (&str, AmountUnit)) -> Self {
        Self {
            currency: value.0.to_string(),
            value: value.1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AmountUnit(u64);

impl AmountUnit {
    pub fn from_f64(v: f64, decimal: u32) -> Result<AmountUnit, AmountUnitFloatError> {
        if !v.is_finite() {
            return Err(AmountUnitFloatError::NotFinite);
        }
        if v < 0.0 {
            return Err(AmountUnitFloatError::Negative);
        }

        let factor = 10f64.powi(decimal as i32);
        let scaled = (v * factor) as u64;

        Ok(AmountUnit(scaled))
    }

    pub fn to_f64(&self, decimal: u32) -> f64 {
        let factor = 10f64.powi(decimal as i32);

        (self.0 as f64) / factor
    }
}

impl From<u64> for AmountUnit {
    fn from(v: u64) -> Self {
        AmountUnit(v)
    }
}

impl From<usize> for AmountUnit {
    fn from(v: usize) -> Self {
        AmountUnit(v as u64)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPaypalSummary {
    pub payer: PaypalPayerDetails,
    pub shipping: PaypalShippingDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaypalPayerDetails {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaypalShippingDetails {
    pub full_name: Option<String>,

    pub address1: String,
    pub address2: Option<String>,
    pub city: String,
    pub country: String,
    pub state: String,
    pub zip_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCardSummary {
    /// Card BIN
    pub card_bin: String,

    /// Expiry month
    pub expiry_month: String,

    /// Maturity year
    pub expiry_year: String,

    /// The last four digits of the card
    #[serde(rename = "last4")]
    pub last_four: String,
}
