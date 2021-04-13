# Cosmos Rust

This is a place for commonly shared rust resources related to the Cosmos ecosystem.

## Crates

| Name                 | Description                 | crates.io | docs.rs | CI Build |
|----------------------|-----------------------------|-----------|---------|----------|
| [`cosmos‑sdk`]       | Cosmos SDK for Rust         | ![crates.io](https://img.shields.io/crates/v/cosmos-sdk.svg?logo=rust) | ![docs.rs](https://docs.rs/cosmos-sdk-rs/badge.svg) | ![CI](https://github.com/cosmos/cosmos-rust/workflows/cosmos-sdk-rs/badge.svg) |
| [`cosmos‑sdk‑proto`] | Proto and gRPC definitions  | ![crates.io](https://img.shields.io/crates/v/cosmos-sdk-proto.svg?logo=rust) | ![docs.rs](https://docs.rs/cosmos-sdk-proto/badge.svg) | ![CI](https://github.com/cosmos/cosmos-rust/workflows/cosmos-sdk-proto/badge.svg) |

## Merge Policy

The goal of this repository is to create a place for upstream consensus in the Cosmos Rust community. Different applications will have different requirements from libraries, this repo should strive to contain only code that is useful and actively used by more than one organization.

Current maintainers of this repo include [InformalSystems](https://github.com/informalsystems), [Iqlusion](https://github.com/iqlusioninc), [Confio](https://github.com/confio), and [Althea](https://github.com/althea-net)

A pull request should be approved by representatives from at least two of those organizations
before being merged. In order to update membership or update these rules a pull request changing
this description must be approved.

## Branch etiquette

Branches in this repo should be of the format git-username/branch-name in order to reduce
clutter and prevent collisions. There is an installed git hook that will automatically enforce
that branches at least contain a /

## Building Proto files from source

The single proto-build crate in this repo clones and rebuilds proto files for
all other crates, simply make the required edits in [main.rs](proto-build/main.rs) and run

    cargo run

## Minimum Supported Rust Version

Rust **1.48**

[//]: # "crates"

[`cosmos‑sdk`]: https://github.com/cosmos/cosmos-rust/tree/main/cosmos-sdk-rs
[`cosmos‑sdk‑proto`]: https://github.com/cosmos/cosmos-rust/tree/main/cosmos-sdk-proto
