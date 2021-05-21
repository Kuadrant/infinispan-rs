use serde::{Deserialize, Serialize};

const DEFAULT_CONCURRENCY_LEVEL: i32 = 1000;
const DEFAULT_ACQUIRE_TIMEOUT: i32 = 15000;

#[derive(Serialize, Deserialize)]
pub struct LocalCache {
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
