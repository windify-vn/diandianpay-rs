mod endpoint;
pub mod request;
pub mod response;
pub mod schema;

#[derive(Debug, thiserror::Error)]
pub enum AmountUnitFloatError {
    #[error("amount must be finite (not NaN/Inf)")]
    NotFinite,
    #[error("amount must be non-negative")]
    Negative,
}
