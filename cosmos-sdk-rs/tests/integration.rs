//! Integration test which submits transactions to a local `gaia` node.
//!
//! Requires Docker.

#![cfg(feature = "rpc")]

use cosmos_sdk::{
    bank::MsgSend,
    crypto::secp256k1,
    rpc,
    rpc::Client,
    tx::{self, AccountNumber, Fee, MsgType, SignDoc, SignerInfo, Tx},
    Coin,
};
use std::{convert::TryFrom, ffi::OsStr, panic, process, str, time::Duration};
use tokio::time;

/// Name of the Docker image (on Docker Hub) to use
const DOCKER_IMAGE: &str = "jackzampolin/gaiatest";

/// Chain ID to use for tests
const CHAIN_ID: &str = "cosmos-sdk-test";

/// RPC port
const RPC_PORT: u16 = 26657;

/// Expected account number
const ACCOUNT_NUMBER: AccountNumber = 1;

/// Bech32 prefix for an account
const ACCOUNT_PREFIX: &str = "cosmos";

/// Denom name
const DENOM: &str = "samoleans";

/// Example memo
const MEMO: &str = "test memo";

#[test]
fn msg_send() {
    let sender_private_key = secp256k1::SigningKey::random();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();

    let recipient_private_key = secp256k1::SigningKey::random();
    let recipient_account_id = recipient_private_key
        .public_key()
        .account_id(ACCOUNT_PREFIX)
        .unwrap();

    let amount = Coin {
        amount: 1u8.into(),
        denom: DENOM.parse().unwrap(),
    };

    let msg_send = MsgSend {
        from_address: sender_account_id.clone(),
        to_address: recipient_account_id,
        amount: vec![amount.clone()],
    }
    .to_msg()
    .unwrap();

    let chain_id = CHAIN_ID.parse().unwrap();
    let sequence_number = 0;
    let gas = 100_000;
    let fee = Fee::from_amount_and_gas(amount, gas);
    let timeout_height = 9001u16;

    let tx_body = tx::Body::new(vec![msg_send], MEMO, timeout_height);
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, ACCOUNT_NUMBER).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

    let docker_args = [
        "-d",
        "-p",
        &format!("{}:{}", RPC_PORT, RPC_PORT),
        DOCKER_IMAGE,
        CHAIN_ID,
        &sender_account_id.to_string(),
    ];

    docker_run(&docker_args, || {
        init_tokio_runtime().block_on(async {
            let rpc_address = format!("http://localhost:{}", RPC_PORT);
            let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();

            wait_for_first_block(&rpc_client).await;

            let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();

            if tx_commit_response.check_tx.code.is_err() {
                panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
            }

            if tx_commit_response.deliver_tx.code.is_err() {
                panic!("deliver_tx failed: {:?}", tx_commit_response.deliver_tx);
            }

            let tx = poll_for_tx(&rpc_client, &tx_commit_response.hash).await;
            assert_eq!(&tx_body, &tx.body);
            assert_eq!(&auth_info, &tx.auth_info);
        })
    });
}

/// Initialize Tokio runtime
fn init_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

/// Wait for the node to produce the first block
async fn wait_for_first_block(rpc_client: &rpc::HttpClient) {
    rpc_client
        .wait_until_healthy(Duration::from_secs(5))
        .await
        .unwrap();

    let mut attempts_remaining = 25;

    while let Err(e) = rpc_client.latest_block().await {
        if e.code() != rpc::error::Code::ParseError {
            panic!("unexpected error waiting for first block: {}", e);
        }

        if attempts_remaining == 0 {
            panic!("timeout waiting for first block");
        }

        attempts_remaining -= 1;
        time::sleep(Duration::from_millis(200)).await;
    }
}

/// Wait for a transaction with the given hash to appear in the blockchain
async fn poll_for_tx(rpc_client: &rpc::HttpClient, tx_hash: &tx::Hash) -> Tx {
    // Look up the transaction by its hash
    let tx_query =
        rpc::query::Query::from(rpc::query::EventType::Tx).and_eq("tx.hash", tx_hash.to_string());

    let attempts = 5;

    for _ in 0..attempts {
        let tx_response = rpc_client
            .tx_search(tx_query.clone(), false, 1, 1, rpc::Order::Ascending)
            .await
            .unwrap();

        if tx_response.total_count == 1 {
            return Tx::try_from(tx_response.txs[0].tx.as_bytes()).unwrap();
        }
    }

    panic!("couldn't find transaction after {} attempts!", attempts);
}

/// Invoke `docker run` with the given arguments, calling the provided function
/// after the container has booted and terminating the container after the
/// provided function completes, catching panics and propagating them to ensure
/// that the container reliably shuts down.
///
/// Prints log output from the container in the event an error occurred.
fn docker_run<A, S, F, R>(args: A, f: F) -> R
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    F: FnOnce() -> R + panic::UnwindSafe,
{
    let container_id = exec_docker_command("run", args);
    let result = panic::catch_unwind(f);

    if result.is_err() {
        let logs = exec_docker_command("logs", &[&container_id]);

        println!("\n---- docker stdout ----");
        println!("{}", logs);
    }

    exec_docker_command("kill", &[&container_id]);

    match result {
        Ok(res) => res,
        Err(err) => panic::resume_unwind(err),
    }
}

/// Execute a given `docker` command, returning what was written to stdout
/// if the command completed successfully.
///
/// Panics if the `docker` process exits with an error code
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
        .expect(&format!("error invoking `docker {}`", name));

    if !output.status.success() {
        panic!("`docker {}` exited with error status: {:?}", name, output);
    }

    str::from_utf8(&output.stdout)
        .expect("UTF-8 error decoding docker output")
        .trim_end()
        .to_owned()
}
