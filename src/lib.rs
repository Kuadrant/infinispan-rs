use std::convert::TryFrom;

use reqwest::Response;

use crate::request::ToHttpRequest;

pub mod request;

#[derive(Debug)]
pub struct Infinispan {
    base_url: String,
    http_client: reqwest::Client,
    basic_auth_encoded_val: String,
}

impl Infinispan {
    pub fn new(
        base_url: impl Into<String>,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Infinispan {
        Infinispan {
            base_url: base_url.into(),
            http_client: reqwest::Client::new(),
            basic_auth_encoded_val: Self::basic_auth_encoded_value(
                username.as_ref(),
                password.as_ref(),
            ),
        }
    }

    pub async fn run<R: ToHttpRequest>(&self, request: &R) -> Result<Response, reqwest::Error> {
        let http_req = request.to_http_req(&self.base_url, &self.basic_auth_encoded_val);

        self.http_client
            .execute(reqwest::Request::try_from(http_req).unwrap())
            .await
    }

    fn basic_auth_encoded_value(username: &str, password: &str) -> String {
        format!(
            "Basic {}",
            base64::encode(format!("{}:{}", username, password))
        )
    }
}
