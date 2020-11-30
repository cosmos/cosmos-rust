# Cosmos Rust

This is a place for commonly shared rust resources related to the Cosmos ecosystem.

## Building Proto files from source

The single proto-build crate in this repo clones and rebuilds proto files for
all other crates, simply make the required edits in [main.rs](proto-build/main.rs) and run

    cargo run
