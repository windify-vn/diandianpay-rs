use crate::framework::response::ApiFailure;
use base64::Engine;
use http::HeaderMap;
use rand::rngs::OsRng;
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, SignatureEncoding, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey, signature};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub merchant_id: String,
    #[serde(with = "crate::framework::serde::rsa::private")]
    pub private_key: RsaPrivateKey,
    #[serde(with = "crate::framework::serde::rsa::public")]
    pub dd_public_key: RsaPublicKey,
}

#[derive(Debug, thiserror::Error)]
pub enum VerifySignatureError {
    #[error("invalid Base64 in signature: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("invalid PKCS#1 v1.5 signature format: {0}")]
    SignatureFormat(#[source] signature::Error),

    #[error("RSA signature verification failed: {0}")]
    Verification(#[source] signature::Error),
}

impl Credentials {
    pub fn headers(&self, body: &str) -> Vec<(&'static str, String)> {
        const TIMEZONE: &str = "Etc/UTC";

        let timestamp = chrono::Utc::now().timestamp_millis().to_string();

        // let request_id = uuid::Uuid::new_v7(Timestamp::now(uuid::NoContext)).to_string();

        let content = format!("{}.{timestamp}.{TIMEZONE}.{body}", self.merchant_id);

        let signing_key = SigningKey::<Sha256>::new(self.private_key.clone());
        let signature = signing_key.sign_with_rng(&mut OsRng, content.as_bytes());

        vec![
            ("timezone", TIMEZONE.into()),
            ("timestamp", timestamp),
            (
                "signature",
                base64::engine::general_purpose::STANDARD.encode(signature.to_bytes()),
            ),
            // ("DD-Request-Id", request_id),
        ]
    }

    pub fn verify_signature(
        &self,
        content: &str,
        signature_b64: &str,
    ) -> Result<(), VerifySignatureError> {
        let verifying_key = VerifyingKey::<Sha256>::new(self.dd_public_key.clone());

        let decoded = base64::engine::general_purpose::STANDARD.decode(signature_b64)?;

        let signature = Signature::try_from(decoded.as_slice())
            .map_err(VerifySignatureError::SignatureFormat)?;

        verifying_key
            .verify(content.as_bytes(), &signature)
            .map_err(VerifySignatureError::Verification)?;

        Ok(())
    }

    pub fn verify_request(&self, headers: &HeaderMap, body: &str) -> Result<(), ApiFailure> {
        let timestamp = headers
            .get("timestamp")
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiFailure::MissingHeader("timestamp".to_string()))?;

        let timezone = headers
            .get("timezone")
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiFailure::MissingHeader("timezone".to_string()))?;

        let signature = headers
            .get("signature")
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiFailure::MissingHeader("signature".to_string()))?;

        let content = format!("{}.{timestamp}.{timezone}.{body}", self.merchant_id);

        self.verify_signature(&content, signature)?;

        Ok(())
    }
}

pub trait SignClient {
    fn sign(self, credentials: &Credentials, body: &str) -> Self;
}
