use serde::{Deserialize, Serialize};

const DEFAULT_CONCURRENCY_LEVEL: i32 = 1_000;
const DEFAULT_ACQUIRE_TIMEOUT: i32 = 15_000;
const DEFAULT_STATE_TRANSFER_TIMEOUT: i32 = 60_000;
const DEFAULT_REMOTE_TIMEOUT: i32 = 17_500;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Local {
    locking: Locking,
    statistics: bool,
}

impl Default for Local {
    fn default() -> Self {
        Self {
            locking: Locking::default(),
            statistics: true,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Distributed {
    mode: String,
    state_transfer: StateTransfer,
    locking: Locking,
    statistics: bool,
}

impl Distributed {
    pub fn create_async() -> Self {
        Self {
            mode: "ASYNC".into(),
            state_transfer: StateTransfer {
                timeout: DEFAULT_STATE_TRANSFER_TIMEOUT,
            },
            locking: Locking::default(),
            statistics: true,
        }
    }

    pub fn create_sync() -> Self {
        Self {
            mode: "SYNC".into(),
            state_transfer: StateTransfer {
                timeout: DEFAULT_STATE_TRANSFER_TIMEOUT,
            },
            locking: Locking::default(),
            statistics: true,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Replicated {
    mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_timeout: Option<i32>,
    state_transfer: StateTransfer,
    locking: Locking,
    statistics: bool,
}

impl Replicated {
    pub fn create_async() -> Self {
        Self {
            mode: "ASYNC".into(),
            remote_timeout: None,
            state_transfer: StateTransfer {
                timeout: DEFAULT_STATE_TRANSFER_TIMEOUT,
            },
            locking: Locking::default(),
            statistics: true,
        }
    }

    pub fn create_sync() -> Self {
        Self {
            mode: "SYNC".into(),
            remote_timeout: Some(DEFAULT_REMOTE_TIMEOUT),
            state_transfer: StateTransfer {
                timeout: DEFAULT_STATE_TRANSFER_TIMEOUT,
            },
            locking: Locking::default(),
            statistics: true,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Invalidation {
    mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_timeout: Option<i32>,
    locking: Locking,
    statistics: bool,
}

impl Invalidation {
    pub fn create_async() -> Self {
        Self {
            mode: "ASYNC".into(),
            remote_timeout: None,
            locking: Locking::default(),
            statistics: true,
        }
    }

    pub fn create_sync() -> Self {
        Self {
            mode: "SYNC".into(),
            remote_timeout: Some(DEFAULT_REMOTE_TIMEOUT),
            locking: Locking::default(),
            statistics: true,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Locking {
    concurrency_level: i32,
    acquire_timeout: i32,
    striping: bool,
}

impl Default for Locking {
    fn default() -> Self {
        Self {
            concurrency_level: DEFAULT_CONCURRENCY_LEVEL,
            acquire_timeout: DEFAULT_ACQUIRE_TIMEOUT,
            striping: false,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
struct StateTransfer {
    timeout: i32,
}
