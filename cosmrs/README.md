# CosmRS: Cosmos Wallet and SDK for Rust

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Framework for building [Cosmos] blockchain applications in Rust, modeled off
of the [Cosmos SDK for Golang].

## About

This library is presently designed to serve as a *client* for interacting
with the Golang implementation of the Cosmos SDK, providing things like wallet
functionality such as transaction signing, and a builder/parser for Cosmos SDK
formatted transaction messages.

It does not implement server-side functionality (yet), such as hooks
and message passing.

## Features

- [CosmWasm]: messages used by smart contracts written using CosmWasm
- [Staking]: support for staking with validators
- [Transactions]: build, sign, and/or parse Cosmos SDK transactions

[Cosmos]: https://cosmos.network/
[Cosmos SDK for Golang]: https://github.com/cosmos/cosmos-sdk

## Minimum Supported Rust Version

This crate is supported on Rust **1.72** or newer.

[//]: # "badges"
[crate-image]: https://buildstats.info/crate/cosmrs
[crate-link]: https://crates.io/crates/cosmrs
[docs-image]: https://docs.rs/cosmrs/badge.svg
[docs-link]: https://docs.rs/cosmrs/
[build-image]: https://github.com/cosmos/cosmos-rust/workflows/cosmrs/badge.svg
[build-link]: https://github.com/cosmos/cosmos-rust/actions/workflows/cosmrs.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/cosmos/cosmos-rust/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.72+-blue.svg

[//]: # "general links"
[Cosmos]: https://cosmos.network/
[Cosmos SDK for Golang]: https://github.com/cosmos/cosmos-sdk
[CosmWasm]: https://cosmwasm.com/
[Staking]: https://docs.cosmos.network/master/modules/staking/
[Transactions]: https://docs.cosmos.network/master/core/transactions.html
