# Change Log

## 0.3.0 - 2021-12-24

### Changed

- Move to edition 2021.
([#22](https://github.com/kuadrant/infinispan-rs/pull/22)).
- Require reqwest 0.11 to use the Tokio 1.X async runtime
([#22](https://github.com/kuadrant/infinispan-rs/pull/22)).

## 0.2.0 - 2021-05-19

### Added

- Added more operations for `Counter`: `get_config`, `decrement`, `reset`,
`compare_and_set`, and `compare_and_swap`
([#16](https://github.com/kuadrant/infinispan-rs/pull/16)).
- Added more operations for `Cache`: `get`, `get_config`, `keys`, `clear`,
`size`, `stats`, and `list`
([#17](https://github.com/kuadrant/infinispan-rs/pull/17)).
- The `Infinispan` struct derives `Clone`
([#18](https://github.com/kuadrant/infinispan-rs/pull/18)).

## 0.1.0 - 2021-04-26

First release.

