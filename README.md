# infinispan-rs

![Github
Workflow](https://github.com/3scale-labs/infinispan-rs/workflows/Infinispan-rs/badge.svg)
[![docs.rs](https://docs.rs/infinispan/badge.svg)](https://docs.rs/infinispan)
[![Crates.io](https://img.shields.io/crates/v/infinispan)](https://crates.io/crates/infinispan)

infinispan-rs is a Rust client for the [Infinispan REST
API](https://infinispan.org/docs/stable/titles/rest/rest.html). For now, it
implements a small part of the API.

- [**Getting started**](#getting-started)
- [**Development**](#development)
- [**License**](#license)

## Getting started

```rust
// Create a client
let client = Infinispan::new("http://localhost:11222", "username", "password");

// Create a cache
let req = request::caches::create_local("some_cache");
let _ = client.run(&req).await.unwrap();

// Create an entry
let req = request::entries::create("some_cache", "some_entry").with_value("a_value".into());
let _ = client.run(&req).await.unwrap();

// Read the entry
let req = request::entries::get("some_cache", "some_entry");
let resp = client.run(&req).await.unwrap();

// resp is an instance of `reqwest::Response`
assert!(resp.status().is_success());
assert_eq!("a_value", resp.text_with_charset("utf-8").await.unwrap());
```

## Development

### Build

```bash
cargo build
```

### Run the tests

Some tests need Infinispan running in `localhost:11222`. You can run it in
Docker with:
```bash
docker run -it -p 11222:11222 -e USER="username" -e PASS="password"  infinispan/server:11.0.9.Final
```

Then, run the tests:

```bash
cargo test
```

## License

[Apache 2.0 License](LICENSE)
