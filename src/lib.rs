//! infinispan-rs is a Rust client for the [Infinispan REST
//! API](https://infinispan.org/docs/stable/titles/rest/rest.html).
//!
//! # Basic operation
//!
//! ```no_run
//! use infinispan::Infinispan;
//! use infinispan::errors::InfinispanError;
//! use infinispan::request;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a client
//!     let client = Infinispan::new("http://localhost:11222", "username", "password");
//!
//!     // Create a cache
//!     let req = request::caches::create_local("some_cache");
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Create an entry
//!     let req = request::entries::create("some_cache", "some_entry").with_value("a_value".into());
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Read the entry
//!     let req = request::entries::get("some_cache", "some_entry");
//!     let resp = client.run(&req).await.unwrap();
//!
//!     // resp is an instance of `reqwest::Response`
//!     assert!(resp.status().is_success());
//!     assert_eq!("a_value", resp.text_with_charset("utf-8").await.unwrap());
//! }
//!
//! ```
//!
//! infinispan-rs supports requests to manage Caches, Entries, and Counters, but
//! for now, it only implements a reduced subset of the REST API. Here are some
//! examples:
//!
//! ## Caches
//!
//! ```no_run
//! use infinispan::Infinispan;
//! use infinispan::request;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Infinispan::new("http://localhost:11222", "username", "password");
//!
//!     // Create a cache
//!     let req = request::caches::create_local("some_cache");
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Delete a cache
//!     let req = request::caches::delete("some_cache");
//!     let _ = client.run(&req).await.unwrap();
//! }
//! ```
//!
//! ## Entries
//!
//! ```no_run
//! use infinispan::Infinispan;
//! use infinispan::request;
//! use std::time::Duration;
//! use http::StatusCode;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Infinispan::new("http://localhost:11222", "username", "password");
//!
//!     // Create a cache
//!     let req = request::caches::create_local("some_cache");
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Create an entry
//!     let req = request::entries::create("some_cache", "some_entry").with_value("a_value".into());
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Create an entry with a value and TTL
//!     let req = request::entries::create("some_cache", "some_entry_with_ttl")
//!         .with_value("a_value".into()).with_ttl(Duration::from_secs(5));
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Check if an entry exists
//!     let req = request::entries::exists("some_cache", "some_entry");
//!     let resp = client.run(&req).await.unwrap();
//!     assert!(resp.status().is_success());
//!
//!     let req = request::entries::exists("some_cache", "non_existing");
//!     let resp = client.run(&req).await.unwrap();
//!     assert_eq!(StatusCode::NOT_FOUND, resp.status());
//!
//!     // Update an entry
//!     let req = request::entries::update("some_cache", "some_entry", "new_val");
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Delete an entry
//!     let req = request::entries::delete("some_cache", "some_entry");
//!     let _ = client.run(&req).await.unwrap();
//! }
//! ```
//!
//! ## Counters
//!
//! ```no_run
//! use infinispan::Infinispan;
//! use infinispan::request;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Infinispan::new("http://localhost:11222", "username", "password");
//!
//!     // Create a weak counter
//!     let req = request::counters::create_weak("some_counter").with_value(100);
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Increment a counter
//!     let req = request::counters::increment("some_counter");
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Increment a counter by a given delta
//!     let req = request::counters::increment("some_counter").by(10);
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Delete a counter
//!     let req = request::counters::delete("some_counter");
//!     let _ = client.run(&req).await.unwrap();
//!
//!     // Create a strong counter
//!     let req = request::counters::create_strong("some_strong_counter");
//!     let _ = client.run(&req).await.unwrap();
//! }
//! ```
//!

use std::convert::TryFrom;

use reqwest::Response;

use crate::errors::InfinispanError;
use crate::request::ToHttpRequest;

pub mod errors;
pub mod request;

#[derive(Debug, Clone)]
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
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client: reqwest::Client::new(),
            basic_auth_encoded_val: Self::basic_auth_encoded_value(
                username.as_ref(),
                password.as_ref(),
            ),
        }
    }

    pub async fn run<R: ToHttpRequest>(&self, request: &R) -> Result<Response, InfinispanError> {
        let http_req = request.to_http_req(&self.base_url, &self.basic_auth_encoded_val);

        let res = self
            .http_client
            .execute(reqwest::Request::try_from(http_req)?)
            .await?;

        Ok(res)
    }

    fn basic_auth_encoded_value(username: &str, password: &str) -> String {
        format!(
            "Basic {}",
            base64::encode(format!("{}:{}", username, password))
        )
    }
}
