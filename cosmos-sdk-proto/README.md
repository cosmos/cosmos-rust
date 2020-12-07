# cosmos-sdk-proto

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Rust crate for interacting with Cosmos SDK
[Protobuf structs](https://github.com/cosmos/cosmos-sdk/tree/master/proto/).

The goal of this crate is to provide complete proto struct definitions for interacting
with a CosmosSDK blockchain. Currently this crate only provides a minority of the many
total structs exported by proto files. Pull requests to expand coverage are welcome.

[Documentation][docs-link]

## Requirements

- Rust 1.48+
- Cosmos SDK (downloaded automatically)

[//]: # "badges"
[crate-image]: https://img.shields.io/crates/v/cosmos-rust.svg
[crate-link]: https://crates.io/crates/cosmos-rust
[docs-image]: https://docs.rs/cosmos-rust/badge.svg
[docs-link]: https://docs.rs/cosmos-rust/
[build-image]: https://github.com/cosmos/cosmos-rust/workflows/Rust/badge.svg
[build-link]: https://github.com/cosmos/cosmos-rust/actions?query=workflow%3ARust
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/cosmos/cosmos-rust/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.48+-blue.svg
[//]: # "general links"
[cosmos sdk]: https://github.com/cosmos/cosmos-sdk
