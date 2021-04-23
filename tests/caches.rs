mod helpers;

#[cfg(test)]
mod caches {
    use crate::helpers::*;
    use http::StatusCode;
    use infinispan::request::caches;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn create_local() {
        let cache_name = "test_cache";

        let _ = run(&caches::create_local(cache_name)).await;
        let resp = run(&caches::exists(cache_name)).await;

        assert!(resp.status().is_success());
    }

    #[tokio::test]
    #[serial]
    async fn delete() {
        let cache_name = "test_cache";

        let _ = run(&caches::create_local(cache_name)).await;

        let _ = run(&caches::delete(cache_name)).await;
        let resp = run(&caches::exists(cache_name)).await;

        assert_eq!(StatusCode::NOT_FOUND, resp.status());
    }
}
