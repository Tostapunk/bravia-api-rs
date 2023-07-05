//! APIs that are related to encryption.
//!
//! #
//! For details on encryption specifications,
//! please see [Sony's documentation](https://pro-bravia.sony.net/develop/integrate/rest-api/doc/Data-Encryption_401146660/index.html).

use crate::{error::Result, Bravia, RequestBodyBuilder, RequestBuilder};

const ENDPOINT: &str = "encryption";

/// Provides access to encryption service APIs.
pub struct EncryptionService<'a>(&'a Bravia);

impl<'a> EncryptionService<'a> {
    pub fn new(bravia: &'a Bravia) -> Self {
        Self(bravia)
    }

    /// Requests the device to provide an RSA public key for encryption.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_public_key(&self) -> Result<String> {
        let body = RequestBodyBuilder::default()
            .id(1)
            .method("getPublicKey")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .get("publicKey".into())
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }
}
