pub mod rsa {
    pub mod private {
        use base64::Engine;
        use rsa::RsaPrivateKey;
        use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S>(key: &RsaPrivateKey, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let der = key.to_pkcs8_der().map_err(serde::ser::Error::custom)?;

            let encoded = base64::engine::general_purpose::STANDARD.encode(der.as_bytes());

            encoded.serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<RsaPrivateKey, D::Error>
        where
            D: Deserializer<'de>,
        {
            let encoded: &str = Deserialize::deserialize(deserializer)?;
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(encoded.as_bytes())
                .map_err(serde::de::Error::custom)?;

            RsaPrivateKey::from_pkcs8_der(decoded.as_slice()).map_err(serde::de::Error::custom)
        }
    }

    pub mod public {
        use base64::Engine;
        use rsa::RsaPublicKey;
        use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S>(key: &RsaPublicKey, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let der = key.to_public_key_der().map_err(serde::ser::Error::custom)?;

            let encoded = base64::engine::general_purpose::STANDARD.encode(der.as_bytes());

            encoded.serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<RsaPublicKey, D::Error>
        where
            D: Deserializer<'de>,
        {
            let encoded: &str = Deserialize::deserialize(deserializer)?;
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(encoded.as_bytes())
                .map_err(serde::de::Error::custom)?;

            RsaPublicKey::from_public_key_der(decoded.as_slice()).map_err(serde::de::Error::custom)
        }
    }
}
