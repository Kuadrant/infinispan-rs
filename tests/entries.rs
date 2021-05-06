mod helpers;

#[cfg(test)]
mod entries {
    use crate::helpers::{read_body, run};
    use http::StatusCode;
    use infinispan::request::caches;
    use infinispan::request::entries;
    use serial_test::serial;

    const TEST_CACHE_NAME: &str = "test_cache";

    #[tokio::test]
    #[serial]
    async fn create() {
        setup().await;

        let entry_name = "test_entry";

        let _ = run(&entries::create(TEST_CACHE_NAME, entry_name)).await;
        let resp = run(&entries::exists(TEST_CACHE_NAME, entry_name)).await;

        assert!(resp.status().is_success());
    }

    #[tokio::test]
    #[serial]
    async fn create_with_value() {
        setup().await;

        let entry_name = "test_entry";
        let entry_value = "some_value";

        let _ =
            run(&entries::create(TEST_CACHE_NAME, entry_name).with_value(entry_value.into())).await;
        let resp = run(&entries::get(TEST_CACHE_NAME, entry_name)).await;

        assert_eq!(entry_value, read_body(resp).await);
    }

    #[tokio::test]
    #[serial]
    async fn exists() {
        setup().await;

        let existing_key_name = "existing";
        let non_existing_key_name = "non_existing";

        let _ = run(&entries::create(TEST_CACHE_NAME, existing_key_name)).await;

        let resp_existing = run(&entries::exists(TEST_CACHE_NAME, existing_key_name)).await;
        let resp_non_existing = run(&entries::exists(TEST_CACHE_NAME, non_existing_key_name)).await;

        assert!(resp_existing.status().is_success());
        assert_eq!(StatusCode::NOT_FOUND, resp_non_existing.status());
    }

    #[tokio::test]
    #[serial]
    async fn update() {
        setup().await;

        let entry_name = "test_entry";
        let new_value = "new_value";

        let _ =
            run(&entries::create(TEST_CACHE_NAME, entry_name)
                .with_value("some_initial_value".into()))
            .await;

        let _ = run(&entries::update(TEST_CACHE_NAME, entry_name, new_value)).await;
        let resp = run(&entries::get(TEST_CACHE_NAME, entry_name)).await;

        assert_eq!(new_value, read_body(resp).await);
    }

    #[tokio::test]
    #[serial]
    async fn delete() {
        setup().await;

        let entry_name = "test_entry";

        let _ = run(&entries::create(TEST_CACHE_NAME, entry_name)).await;
        let resp = run(&entries::exists(TEST_CACHE_NAME, entry_name)).await;
        assert!(resp.status().is_success());

        let _ = run(&entries::delete(TEST_CACHE_NAME, entry_name)).await;
        let resp = run(&entries::exists(TEST_CACHE_NAME, entry_name)).await;
        assert_eq!(StatusCode::NOT_FOUND, resp.status());
    }

    async fn setup() {
        let _ = run(&caches::delete(TEST_CACHE_NAME)).await;
        let _ = run(&caches::create_local(TEST_CACHE_NAME)).await;
    }
}
