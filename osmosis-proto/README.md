# osmosis-proto

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Rust crate for interacting with [Protobufs] defined by the [Osmosis] blockchain.
Only includes Osmosis specific types. To get general cosmos types also import `cosmos-sdk-proto`.

[Documentation][docs-link]

## Build

`cd osmosis-proto-build`  
`cargo run`

## Cargo.toml
`osmosis-proto = { path = "../cosmos-rust/osmosis-proto", features = ["grpc-transport", "osmosis"] }`


[//]: # "badges"
[crate-image]: https://buildstats.info/crate/osmosis-proto
[crate-link]: https://crates.io/crates/osmosis-proto
[docs-image]: https://docs.rs/osmosis-proto/badge.svg
[docs-link]: https://docs.rs/osmosis-proto/
[build-image]: https://github.com/cosmos/cosmos-rust/workflows/osmosis-proto/badge.svg
[build-link]: https://github.com/cosmos/cosmos-rust/actions/workflows/osmosis-proto.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/cosmos/cosmos-rust/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg

[//]: # "general links"
[Protobufs]: https://github.com/osmosis-labs/osmosis/tree/main/proto/osmosis
[Osmosis]: https://github.com/osmosis-labs/osmosis/
