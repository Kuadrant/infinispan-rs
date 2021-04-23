use std::collections::HashMap;

use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::Request as HttpRequest;

pub mod caches;
pub mod counters;
pub mod entries;

#[derive(Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
}

impl Method {
    pub fn as_str(&self) -> &str {
        use Method::*;

        match self {
            Get => "GET",
            Head => "HEAD",
            Post => "POST",
            Put => "PUT",
            Delete => "DELETE",
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path_and_query: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub trait ToHttpRequest {
    fn to_http_req(
        &self,
        base_url: impl AsRef<str>,
        basic_auth_encoded: impl AsRef<str>,
    ) -> HttpRequest<String>;
}

impl Request {
    pub fn new(
        method: impl Into<Method>,
        path_and_query: impl Into<String>,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Request {
        Request {
            method: method.into(),
            path_and_query: path_and_query.into(),
            headers,
            body,
        }
    }
}

impl ToHttpRequest for Request {
    fn to_http_req(
        &self,
        base_url: impl AsRef<str>,
        basic_auth_encoded: impl AsRef<str>,
    ) -> HttpRequest<String> {
        let mut http_req = HttpRequest::builder()
            .method(self.method.as_str())
            .uri(format!("{}{}", base_url.as_ref(), self.path_and_query));

        http_req = http_req
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, basic_auth_encoded.as_ref());

        for (header_name, header_val) in &self.headers {
            http_req = http_req.header(header_name.as_str(), header_val);
        }

        http_req
            .body(self.body.as_ref().map_or("".to_string(), |b| b.to_string()))
            .unwrap()
    }
}
