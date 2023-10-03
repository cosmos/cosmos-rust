# cosmos-sdk-proto

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Rust crate for interacting with [Protobufs] defined by the [Cosmos SDK].

The goal of this crate is to provide complete proto struct definitions for interacting
with a Cosmos SDK blockchain.

Currently, this crate only provides a subset of the many total structs exported by
Cosmos SDK proto files.

Pull requests to expand coverage are welcome.

[Documentation][docs-link]

## Minimum Supported Rust Version

This crate is supported on Rust **1.72** or newer.

[//]: # "badges"
[crate-image]: https://buildstats.info/crate/cosmos-sdk-proto
[crate-link]: https://crates.io/crates/cosmos-sdk-proto
[docs-image]: https://docs.rs/cosmos-sdk-proto/badge.svg
[docs-link]: https://docs.rs/cosmos-sdk-proto/
[build-image]: https://github.com/cosmos/cosmos-rust/workflows/cosmos-sdk-proto/badge.svg
[build-link]: https://github.com/cosmos/cosmos-rust/actions/workflows/cosmos-sdk-proto.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/cosmos/cosmos-rust/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.72+-blue.svg

[//]: # "links"
[Protobufs]: (https://github.com/cosmos/cosmos-sdk/tree/master/proto/)
[Cosmos SDK]: https://github.com/cosmos/cosmos-sdk
