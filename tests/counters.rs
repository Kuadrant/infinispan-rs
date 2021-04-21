// These tests cannot be run in parallel. We use the "serial_test" crate to run
// them one by one.

mod helpers;

#[cfg(test)]
mod counters {
    use crate::helpers::*;
    use http::StatusCode;
    use infinispan::request::counters;
    use reqwest::Response;
    use serial_test::serial;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[tokio::test]
    #[serial]
    async fn create_weak_counter() {
        cleanup().await;

        let counter_name = "test_counter";

        let _ = run(counters::create_weak(counter_name)).await;
        let resp = run(counters::get(counter_name)).await;

        assert!(resp.status().is_success());
        assert_eq!("0", resp.bytes().await.unwrap()); // Default counter value is 0
    }

    #[tokio::test]
    #[serial]
    async fn create_weak_counter_with_value() {
        cleanup().await;

        let counter_name = "test_counter";
        let counter_val = 10;

        let _ = run(counters::create_weak(counter_name).with_value(counter_val)).await;
        let resp = run(counters::get(counter_name)).await;

        assert!(resp.status().is_success());
        assert_eq!(counter_val.to_string(), resp.bytes().await.unwrap());
    }

    #[tokio::test]
    #[serial]
    async fn create_strong_counter() {
        cleanup().await;

        let counter_name = "test_counter";

        let _ = run(counters::create_strong(counter_name)).await;
        let resp = run(counters::get(counter_name)).await;

        assert!(resp.status().is_success());
        assert_eq!("0", resp.bytes().await.unwrap()); // Default counter value is 0
    }

    #[tokio::test]
    #[serial]
    async fn create_strong_counter_with_value() {
        cleanup().await;

        let counter_name = "test_counter";
        let counter_val = 10;

        let _ = run(counters::create_strong(counter_name).with_value(counter_val)).await;
        let resp = run(counters::get(counter_name)).await;

        assert!(resp.status().is_success());
        assert_eq!(counter_val.to_string(), resp.bytes().await.unwrap());
    }

    #[tokio::test]
    #[serial]
    async fn increment() {
        cleanup().await;

        let counter_name = "test_counter";

        let _ = run(counters::create_strong(counter_name)).await;
        let _ = run(counters::increment(counter_name)).await;
        let resp = run(counters::get(counter_name)).await;

        assert!(resp.status().is_success());
        assert_eq!("1", resp.bytes().await.unwrap());
    }

    #[tokio::test]
    #[serial]
    async fn increment_with_delta() {
        cleanup().await;

        let counter_name = "test_counter";

        let _ = run(counters::create_strong(counter_name).with_value(1)).await;
        let _ = run(counters::increment(counter_name).by(2)).await;
        let resp = run(counters::get(counter_name)).await;

        assert!(resp.status().is_success());
        assert_eq!("3", resp.bytes().await.unwrap());
    }

    #[tokio::test]
    #[serial]
    async fn delete() {
        cleanup().await;

        let counter_name = "test_counter";

        let _ = run(counters::create_weak(counter_name)).await;
        let _ = run(counters::delete(counter_name)).await;
        let resp = run(counters::get(counter_name)).await;

        assert_eq!(StatusCode::NOT_FOUND, resp.status());
    }

    #[tokio::test]
    #[serial]
    async fn list() {
        cleanup().await;

        let counter_names: HashSet<String> =
            HashSet::from_iter(vec!["counter_1".into(), "counter_2".into()]);

        for counter_name in &counter_names {
            let _ = run(counters::create_weak(counter_name)).await;
        }

        let resp = run(counters::list()).await;

        assert_eq!(counter_names, counter_names_from_list_resp(resp).await);
    }

    async fn cleanup() {
        let resp = run(counters::list()).await;

        for counter_name in counter_names_from_list_resp(resp).await {
            let _ = run(counters::delete(counter_name)).await;
        }
    }

    async fn counter_names_from_list_resp(response: Response) -> HashSet<String> {
        let counter_bytes = response.bytes().await.unwrap();
        let counters_string = std::str::from_utf8(&counter_bytes).unwrap();
        serde_json::from_str(&counters_string).unwrap()
    }
}
