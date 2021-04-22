use crate::request::caches::CACHES_ENDPOINT;
use crate::request::{Method, Request};
use std::collections::HashMap;
use std::time::Duration;

const TTL_HEADER: &str = "timeToLiveSeconds";

pub struct CreateEntryReq {
    cache_name: String,
    entry_name: String,
    value: Option<String>,
    ttl: Option<Duration>,
}

impl CreateEntryReq {
    pub fn new(cache_name: impl Into<String>, entry_name: impl Into<String>) -> CreateEntryReq {
        CreateEntryReq {
            cache_name: cache_name.into(),
            entry_name: entry_name.into(),
            value: None,
            ttl: None,
        }
    }

    pub fn with_value(mut self, value: String) -> CreateEntryReq {
        self.value = Some(value);
        self
    }

    pub fn with_ttl(mut self, ttl: Duration) -> CreateEntryReq {
        self.ttl = Some(ttl);
        self
    }
}

impl From<CreateEntryReq> for Request {
    fn from(request: CreateEntryReq) -> Request {
        let mut headers = HashMap::new();

        if let Some(ttl) = request.ttl {
            headers.insert(TTL_HEADER.into(), ttl.as_secs().to_string());
        }

        Request::new(
            Method::Post,
            entry_url(request.cache_name, request.entry_name),
            headers,
            request.value,
        )
    }
}

pub fn create(cache_name: impl Into<String>, entry_name: impl Into<String>) -> CreateEntryReq {
    CreateEntryReq::new(cache_name, entry_name)
}

pub fn get(cache_name: impl AsRef<str>, entry_name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Get,
        entry_url(cache_name, entry_name),
        HashMap::new(),
        None,
    )
}

pub fn exists(cache_name: impl AsRef<str>, entry_name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Head,
        entry_url(cache_name, entry_name),
        HashMap::new(),
        None,
    )
}

pub fn update(
    cache_name: impl AsRef<str>,
    entry_name: impl AsRef<str>,
    value: impl Into<String>,
) -> Request {
    Request::new(
        Method::Put,
        entry_url(cache_name, entry_name),
        HashMap::new(),
        Some(value.into()),
    )
}

pub fn delete(cache_name: impl AsRef<str>, entry_name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Delete,
        entry_url(cache_name, entry_name),
        HashMap::new(),
        None,
    )
}

fn entry_url(cache_name: impl AsRef<str>, entry_name: impl AsRef<str>) -> String {
    format!(
        "{caches_endpoint}/{cache_name}/{entry_name}",
        caches_endpoint = CACHES_ENDPOINT,
        cache_name = urlencoding::encode(cache_name.as_ref()),
        entry_name = urlencoding::encode(entry_name.as_ref())
    )
}
