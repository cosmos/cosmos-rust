//! Development-related functionality.
//!
//! This module contains support for integration testing against a
//! Cosmos SDK-compatible full node (gaia) running inside of Docker.

#![allow(clippy::panic)]

use crate::{
    rpc::{self, Client},
    tx::Tx,
};
use std::{ffi::OsStr, panic, process, str, time::Duration};
use tendermint::Hash;
use tokio::time;

/// Docker image (on Docker Hub) containing a single-node test environment for
/// [`gaia`], the reference implementation of a Cosmos Hub full node.
///
/// [`gaia`]: https://github.com/cosmos/gaia
pub const GAIA_DOCKER_IMAGE: &str = "jackzampolin/gaiatest";

/// Invoke `docker run` with the given arguments, calling the provided function
/// after the container has booted and terminating the container after the
/// provided function completes, catching panics and propagating them to ensure
/// that the container reliably shuts down.
///
/// Prints log output from the container in the event an error occurred.
pub fn docker_run<A, S, F, R>(args: A, f: F) -> R
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    F: FnOnce() -> R + panic::UnwindSafe,
{
    let container_id = exec_docker_command("run", args);
    let result = panic::catch_unwind(f);

    if result.is_err() {
        let logs = exec_docker_command("logs", [&container_id]);

        println!("\n---- docker stdout ----");
        println!("{}", logs);
    }

    exec_docker_command("kill", [&container_id]);

    match result {
        Ok(res) => res,
        Err(err) => panic::resume_unwind(err),
    }
}

/// Execute a given `docker` command, returning what was written to stdout
/// if the command completed successfully.
///
/// Panics if the `docker` process exits with an error code.
fn exec_docker_command<A, S>(name: &str, args: A) -> String
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = process::Command::new("docker")
        .arg(name)
        .args(args)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap_or_else(|err| panic!("error invoking `docker {}`: {}", name, err));

    if !output.status.success() {
        panic!("`docker {}` exited with error status: {:?}", name, output);
    }

    str::from_utf8(&output.stdout)
        .expect("UTF-8 error decoding docker output")
        .trim_end()
        .to_owned()
}

/// Wait for the node to produce the first block.
///
/// This should be used at the beginning of the test lifecycle to ensure
/// the node is fully booted.
pub async fn poll_for_first_block(rpc_client: &rpc::HttpClient) {
    rpc_client
        .wait_until_healthy(Duration::from_secs(5))
        .await
        .expect("error waiting for RPC to return healthy responses");

    let mut attempts_remaining = 25;

    while let Err(e) = rpc_client.latest_block().await {
        if !matches!(e.detail(), rpc::error::ErrorDetail::Serde(_)) {
            panic!("unexpected error waiting for first block: {:?}", e);
        }

        if attempts_remaining == 0 {
            panic!("timeout waiting for first block");
        }

        attempts_remaining -= 1;
        time::sleep(Duration::from_millis(200)).await;
    }
}

/// Wait for a transaction with the given hash to appear in the blockchain
pub async fn poll_for_tx(rpc_client: &rpc::HttpClient, tx_hash: Hash) -> Tx {
    let attempts = 5;

    // TODO(tarcieri): better conversion or unified `Hash` type, see tendermint-rs#1221
    #[allow(clippy::unwrap_used)]
    let tx_hash = tendermint::Hash::Sha256(tx_hash.as_ref().try_into().unwrap());

    for _ in 0..attempts {
        // TODO(tarcieri): handle not found errors
        if let Ok(tx) = Tx::find_by_hash(rpc_client, tx_hash).await {
            return tx;
        }
    }

    panic!("couldn't find transaction after {} attempts!", attempts);
}
