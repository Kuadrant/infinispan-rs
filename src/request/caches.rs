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

pub fn delete(name: impl AsRef<str>) -> Request {
    Request::new(Method::Delete, cache_url(name), HashMap::new(), None)
}

fn cache_url(name: impl AsRef<str>) -> String {
    format!(
        "{caches_endpoint}/{cache_name}",
        caches_endpoint = CACHES_ENDPOINT,
        cache_name = urlencoding::encode(name.as_ref())
    )
}
