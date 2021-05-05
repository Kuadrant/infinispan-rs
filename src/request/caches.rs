use crate::request::{Method, Request};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

pub(crate) const CACHES_ENDPOINT: &str = "/rest/v2/caches";

const DEFAULT_CONCURRENCY_LEVEL: i32 = 1000;
const DEFAULT_ACQUIRE_TIMEOUT: i32 = 15000;

#[derive(Serialize, Deserialize)]
enum Cache {
    #[serde(rename = "local-cache")]
    LocalCache(LocalCache),
}

#[derive(Serialize, Deserialize)]
struct LocalCache {
    locking: Locking,
    statistics: bool,
}

#[derive(Serialize, Deserialize)]
struct Locking {
    #[serde(rename = "concurrency-level")]
    concurrency_level: i32,
    #[serde(rename = "acquire-timeout")]
    acquire_timeout: i32,
    striping: bool,
}

impl Default for LocalCache {
    fn default() -> Self {
        Self {
            locking: Locking {
                concurrency_level: DEFAULT_CONCURRENCY_LEVEL,
                acquire_timeout: DEFAULT_ACQUIRE_TIMEOUT,
                striping: false,
            },
            statistics: true,
        }
    }
}

#[derive(Debug)]
enum Action {
    Clear,
    Config,
    Keys,
    Size,
    Stats,
}

impl Action {
    pub fn to_query_args(&self) -> String {
        let mut query_args = format!("action={:?}", self);
        query_args.make_ascii_lowercase();
        query_args
    }
}

pub fn create_local(name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Post,
        cache_url(name),
        HashMap::new(),
        Some(json!(Cache::LocalCache(LocalCache::default())).to_string()),
    )
}

pub fn exists(name: impl AsRef<str>) -> Request {
    Request::new(Method::Head, cache_url(name), HashMap::new(), None)
}

pub fn get(name: impl AsRef<str>) -> Request {
    Request::new(Method::Get, cache_url(name), HashMap::new(), None)
}

pub fn get_config(name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Get,
        cache_url_with_action(name, &Action::Config),
        HashMap::new(),
        None,
    )
}

pub fn delete(name: impl AsRef<str>) -> Request {
    Request::new(Method::Delete, cache_url(name), HashMap::new(), None)
}

pub fn keys(name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Get,
        cache_url_with_action(name, &Action::Keys),
        HashMap::new(),
        None,
    )
}

pub fn clear(name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Post,
        cache_url_with_action(name, &Action::Clear),
        HashMap::new(),
        None,
    )
}

pub fn size(name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Get,
        cache_url_with_action(name, &Action::Size),
        HashMap::new(),
        None,
    )
}

pub fn stats(name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Get,
        cache_url_with_action(name, &Action::Stats),
        HashMap::new(),
        None,
    )
}

pub fn list() -> Request {
    Request::new(Method::Get, CACHES_ENDPOINT, HashMap::new(), None)
}

fn cache_url(name: impl AsRef<str>) -> String {
    format!(
        "{caches_endpoint}/{cache_name}",
        caches_endpoint = CACHES_ENDPOINT,
        cache_name = urlencoding::encode(name.as_ref())
    )
}

fn cache_url_with_action(name: impl AsRef<str>, action: &Action) -> String {
    format!("{}?{}", cache_url(name), action.to_query_args())
}
