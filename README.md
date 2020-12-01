# Cosmos Rust

This is a place for commonly shared rust resources related to the Cosmos ecosystem.

## Merge Policy

The goal of this repository is to create a place for upstream consensus in the Cosmos Rust community. Different applications will have different requirements from libraries, this repo should strive to contain only code that is useful and actively used by more than one organization.

Current maintainers of this repo include [InformalSystems](https://github.com/informalsystems), [Iqlusion](https://github.com/iqlusioninc), and [Althea](https://github.com/althea-net)

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
