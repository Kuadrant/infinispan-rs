use crate::request::{Method, Request, ToHttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

const COUNTERS_ENDPOINT: &str = "/rest/v2/counters";

type CounterVal = i64;

#[derive(Debug, Serialize, Deserialize)]
enum Counter {
    #[serde(rename = "weak-counter")]
    Weak(WeakCounter),
    #[serde(rename = "strong-counter")]
    Strong(StrongCounter),
}

#[derive(Debug, Serialize, Deserialize)]
struct WeakCounter {
    #[serde(rename = "initial-value", skip_serializing_if = "Option::is_none")]
    initial_value: Option<CounterVal>,
}

impl WeakCounter {
    pub fn set_value(&mut self, counter_val: CounterVal) {
        self.initial_value = Some(counter_val);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct StrongCounter {
    #[serde(rename = "initial-value", skip_serializing_if = "Option::is_none")]
    initial_value: Option<CounterVal>,
}

impl StrongCounter {
    pub fn set_value(&mut self, counter_val: CounterVal) {
        self.initial_value = Some(counter_val);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CounterType {
    Weak,
    Strong,
}

#[derive(Debug)]
pub struct CreateCounterReq {
    name: String,
    counter: Counter,
}

impl CreateCounterReq {
    pub fn new(name: impl Into<String>, counter_type: CounterType) -> Self {
        let counter = match counter_type {
            CounterType::Weak => Counter::Weak(WeakCounter {
                initial_value: None,
            }),
            CounterType::Strong => Counter::Strong(StrongCounter {
                initial_value: None,
            }),
        };

        Self {
            name: name.into(),
            counter,
        }
    }

    pub fn with_value(mut self, value: CounterVal) -> Self {
        match &mut self.counter {
            Counter::Weak(counter) => counter.set_value(value),
            Counter::Strong(counter) => counter.set_value(value),
        }

        self
    }
}

impl From<&CreateCounterReq> for Request {
    fn from(request: &CreateCounterReq) -> Self {
        Self::new(
            Method::Post,
            counter_path(&request.name),
            HashMap::new(),
            Some(json!(request.counter).to_string()),
        )
    }
}

impl ToHttpRequest for CreateCounterReq {
    fn to_http_req(
        &self,
        base_url: impl AsRef<str>,
        basic_auth_encoded: impl AsRef<str>,
    ) -> http::Request<String> {
        Request::from(self).to_http_req(base_url, basic_auth_encoded)
    }
}

#[derive(Debug)]
pub struct IncrementCounterReq {
    name: String,
    delta: Option<CounterVal>,
}

impl IncrementCounterReq {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            delta: None,
        }
    }

    pub fn by(mut self, delta: CounterVal) -> Self {
        self.delta = Some(delta);
        self
    }

    fn query_with_args(&self) -> String {
        match self.delta {
            Some(delta) => {
                format!("{}?action=add&delta={}", counter_path(&self.name), delta)
            }
            None => {
                format!("{}?action=increment", counter_path(&self.name))
            }
        }
    }
}

impl From<&IncrementCounterReq> for Request {
    fn from(request: &IncrementCounterReq) -> Self {
        Self::new(
            Method::Post,
            request.query_with_args(),
            HashMap::new(),
            None,
        )
    }
}

impl ToHttpRequest for IncrementCounterReq {
    fn to_http_req(
        &self,
        base_url: impl AsRef<str>,
        basic_auth_encoded: impl AsRef<str>,
    ) -> http::Request<String> {
        Request::from(self).to_http_req(base_url, basic_auth_encoded)
    }
}

pub fn create_weak(name: impl Into<String>) -> CreateCounterReq {
    CreateCounterReq::new(name, CounterType::Weak)
}

pub fn create_strong(name: impl Into<String>) -> CreateCounterReq {
    CreateCounterReq::new(name, CounterType::Strong)
}

pub fn get(name: impl AsRef<str>) -> Request {
    Request::new(Method::Get, counter_path(name), HashMap::new(), None)
}

pub fn get_config(name: impl AsRef<str>) -> Request {
    Request::new(Method::Get, counter_config_path(name), HashMap::new(), None)
}

pub fn increment(name: impl Into<String>) -> IncrementCounterReq {
    IncrementCounterReq::new(name)
}

pub fn decrement(name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Post,
        format!("{}?action=decrement", counter_path(name)),
        HashMap::new(),
        None,
    )
}

pub fn reset(name: impl AsRef<str>) -> Request {
    Request::new(
        Method::Post,
        format!("{}?action=reset", counter_path(name)),
        HashMap::new(),
        None,
    )
}

pub fn delete(name: impl AsRef<str>) -> Request {
    Request::new(Method::Delete, counter_path(name), HashMap::new(), None)
}

pub fn compare_and_set(name: impl AsRef<str>, expect: CounterVal, update: CounterVal) -> Request {
    Request::new(
        Method::Post,
        format!(
            "{}?action=compareAndSet&expect={}&update={}",
            counter_path(name),
            expect,
            update
        ),
        HashMap::new(),
        None,
    )
}

pub fn compare_and_swap(name: impl AsRef<str>, expect: CounterVal, update: CounterVal) -> Request {
    Request::new(
        Method::Post,
        format!(
            "{}?action=compareAndSwap&expect={}&update={}",
            counter_path(name),
            expect,
            update
        ),
        HashMap::new(),
        None,
    )
}

pub fn list() -> Request {
    Request::new(Method::Get, COUNTERS_ENDPOINT, HashMap::new(), None)
}

fn counter_path(name: impl AsRef<str>) -> String {
    format!(
        "/{counters_endpoint}/{counter_name}",
        counters_endpoint = COUNTERS_ENDPOINT,
        counter_name = urlencoding::encode(name.as_ref())
    )
}

fn counter_config_path(name: impl AsRef<str>) -> String {
    format!("{}/config", counter_path(name))
}
