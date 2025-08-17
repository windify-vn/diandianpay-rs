use crate::framework::client::ClientConfig;
use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::response::ApiResponseType;
use crate::framework::signature::SignClient;
use crate::framework::{
    Environment,
    response::ApiResult,
    response::{ApiErrors, ApiFailure},
    signature::Credentials,
};
use std::borrow::Cow;
use std::net::SocketAddr;

impl SignClient for reqwest::RequestBuilder {
    fn sign(mut self, credentials: &Credentials, body: &str) -> Self {
        for (k, v) in credentials.headers(body) {
            self = self.header(k, v);
        }
        self
    }
}

pub struct Client {
    environment: Environment,
    credentials: Credentials,
    http_client: reqwest::Client,
}
impl Client {
    pub fn new(
        credentials: Credentials,
        config: ClientConfig,
        environment: Environment,
    ) -> Result<Client, crate::framework::Error> {
        let mut builder = reqwest::Client::builder().default_headers(config.default_headers);

        #[cfg(not(target_arch = "wasm32"))]
        {
            // There is no resolve method in wasm.
            if let Some(address) = config.resolve_ip {
                let url = url::Url::from(&environment);
                builder = builder.resolve(
                    url.host_str()
                        .expect("Environment url should have a hostname"),
                    SocketAddr::new(address, 443),
                );
            }

            // There are no timeouts in wasm. The property is documented as no-op in wasm32.
            builder = builder.timeout(config.http_timeout);
        }

        let http_client = builder.build()?;

        Ok(Client {
            environment,
            credentials,
            http_client,
        })
    }

    pub fn new_with_client(
        client: reqwest::Client,
        credentials: Credentials,
        environment: Environment,
    ) -> Client {
        Client {
            environment,
            credentials,
            http_client: client,
        }
    }

    /// Issue an API request of the given type.
    pub async fn request<Endpoint>(&self, endpoint: &Endpoint) -> ApiResult<Endpoint::ResponseType>
    where
        Endpoint: EndpointSpec + Send + Sync,
        Endpoint::ResponseType: ApiResponseType + Send,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(endpoint.method(), endpoint.url(&self.environment));

        if let Some(body) = endpoint.body() {
            match body {
                RequestBody::Json(json) => {
                    request = request.sign(&self.credentials, &json).body(json);
                }
                RequestBody::Raw(bytes) => {
                    request = request.body(bytes);
                }
                RequestBody::MultiPart(multipart) => {
                    let mut form = reqwest::multipart::Form::new();
                    for (name, part) in multipart.parts() {
                        match part {
                            MultipartPart::Text(text) => {
                                form = form.text(name, text);
                            }
                            MultipartPart::Bytes(bytes) => {
                                form = form.part(name, reqwest::multipart::Part::bytes(bytes));
                            }
                        }
                    }
                    request = request.multipart(form);
                }
            }
            match endpoint.content_type() {
                None | Some(Cow::Borrowed("multipart/form-data")) => {}
                Some(content_type) => {
                    request = request.header(reqwest::header::CONTENT_TYPE, content_type.as_ref());
                }
            }
        }

        let response = request.send().await?;

        let status = response.status();
        if status.is_success() {
            let full_bytes = response.bytes().await?;

            // let text = String::from_utf8_lossy(&full_bytes);
            // println!("{}", text);

            Endpoint::ResponseType::from_response(&full_bytes)
        } else {
            let parsed: Result<ApiErrors, reqwest::Error> = response.json().await;
            let errors = parsed.unwrap_or_default();
            Err(ApiFailure::Error(status, errors))
        }
    }
}
