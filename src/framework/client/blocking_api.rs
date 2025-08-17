use crate::framework::Environment;
use crate::framework::client::ClientConfig;
use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::response::{ApiErrors, ApiFailure, ApiResponseType, ApiResult};
use crate::framework::signature::Credentials;
use crate::framework::signature::SignClient;
use std::borrow::Cow;
use std::net::SocketAddr;

pub struct HttpApiClient {
    environment: Environment,
    credentials: Credentials,
    http_client: reqwest::blocking::Client,
}

impl SignClient for reqwest::blocking::RequestBuilder {
    fn sign(mut self, credentials: &Credentials, body: &str) -> Self {
        for (k, v) in credentials.headers(body) {
            self = self.header(k, v);
        }
        self
    }
}

impl HttpApiClient {
    pub fn new(
        credentials: Credentials,
        config: ClientConfig,
        environment: Environment,
    ) -> Result<HttpApiClient, crate::framework::Error> {
        let mut builder = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers);

        if let Some(address) = config.resolve_ip {
            let url = url::Url::from(&environment);
            builder = builder.resolve(
                url.host_str()
                    .expect("Environment url should have a hostname"),
                SocketAddr::new(address, 443),
            );
        }
        let http_client = builder.build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }

    pub fn new_with_client(
        client: reqwest::blocking::Client,
        credentials: Credentials,
        environment: Environment,
    ) -> HttpApiClient {
        HttpApiClient {
            environment,
            credentials,
            http_client: client,
        }
    }

    pub fn request<Endpoint>(&self, endpoint: &Endpoint) -> ApiResult<Endpoint::ResponseType>
    where
        Endpoint: EndpointSpec + Send + Sync,
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
                    let mut form = reqwest::blocking::multipart::Form::new();
                    for (name, part) in multipart.parts() {
                        match part {
                            MultipartPart::Text(text) => {
                                form = form.text(name, text);
                            }
                            MultipartPart::Bytes(bytes) => {
                                form = form
                                    .part(name, reqwest::blocking::multipart::Part::bytes(bytes));
                            }
                        }
                    }
                    request = request.multipart(form);
                }
            }
            // Reqwest::RequestBuilder::multipart sets the content type for us.
            match endpoint.content_type() {
                None | Some(Cow::Borrowed("multipart/form-data")) => {}
                Some(content_type) => {
                    request = request.header(reqwest::header::CONTENT_TYPE, content_type.as_ref());
                }
            }
        }

        let response = request.send()?;

        let status = response.status();
        if status.is_success() {
            let full_bytes = response.bytes()?;

            // let text = String::from_utf8_lossy(&full_bytes);
            // println!("{}", text);

            Endpoint::ResponseType::from_response(&full_bytes)
        } else {
            let parsed: Result<ApiErrors, reqwest::Error> = response.json();
            let errors = parsed.unwrap_or_default();
            Err(ApiFailure::Error(status, errors))
        }
    }
}
