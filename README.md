# Cosmos Rust

This is a place for commonly shared rust resources related to the Cosmos ecosystem.

## Branch etiquette

Branches in this repo should be of the format git-username/branch-name in order to reduce
clutter and prevent collisions. There is an installed git hook that will automatically enforce
that branches at least contain a /

## Building Proto files from source

The single proto-build crate in this repo clones and rebuilds proto files for
all other crates, simply make the required edits in [main.rs](proto-build/main.rs) and run

    cargo run
