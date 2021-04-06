//! Integration test which submits transactions to a local `gaia` node.
//!
//! Requires Docker.

#![cfg(feature = "rpc")]

use cosmos_tx::{
    bank::MsgSend,
    rpc,
    rpc::Client,
    tx::{self, Fee, MsgType},
    Builder, Coin, SigningKey,
};
use std::{ffi::OsStr, panic, process, str, time::Duration};

/// Name of the Docker image (on Docker Hub) to use
const DOCKER_IMAGE: &str = "jackzampolin/gaiatest";

/// Chain ID to use for tests
const CHAIN_ID: &str = "cosmos-tx-test";

/// RPC port
const RPC_PORT: u16 = 26657;

/// Expected account number
const ACCOUNT_NUMBER: u64 = 1;

/// Bech32 prefix for an account
const ACCOUNT_PREFIX: &str = "cosmos";

/// Denom name
const DENOM: &str = "samoleans";

/// Example memo
const MEMO: &str = "test memo";

#[test]
fn msg_send() {
    let sender_private_key = SigningKey::random();
    let sender_account_id = sender_private_key
        .public_key()
        .account_id(ACCOUNT_PREFIX)
        .unwrap();

    let recipient_private_key = SigningKey::random();
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

    let tx = Builder::new(chain_id, ACCOUNT_NUMBER)
        .sign_tx(tx_body, &sender_private_key, sequence_number, fee)
        .unwrap();

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

            rpc_client
                .wait_until_healthy(Duration::from_secs(5))
                .await
                .unwrap();

            // Workaround for what appears to be a Tendermint race condition.
            // There is a short period after the RPC health check succeeds
            // and the `/genesis` endpoint returns valid data where
            // transactions fail due to an internal SDK panic in the
            // "undefined" code space:
            //
            //     TxResult {
            //         code: Err(111222), data: None, log: Log("panic"), info: Info(""),
            //         gas_wanted: Gas(0), gas_used: Gas(0), events: [],
            //         codespace: Codespace("undefined")
            //     }
            //
            // This should ideally get fixed upstream, however there is
            // presently no tracking issue for this bug.
            //
            // If we see it pop up locally or in CI, perhaps we could find
            // some way of polling readiness via RPC, or retrying when this
            // specific error is encountered.
            // TODO(tarcieri): open upstream issue and remove this hack when fixed
            tokio::time::sleep(Duration::from_secs(2)).await;

            let tx_commit_response = tx.broadcast_commit(&rpc_client).await.unwrap();

            if tx_commit_response.check_tx.code.is_err() {
                panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
            }

            if tx_commit_response.deliver_tx.code.is_err() {
                panic!("deliver_tx failed: {:?}", tx_commit_response.deliver_tx);
            }

            // TODO(tarcieri): look up transaction by hash and test transaction parsing
        });
    });
}

/// Initialize Tokio runtime
fn init_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

/// Invoke `docker run` with the given arguments, calling the provided function
/// after the container has booted and terminating the container after the
/// provided function completes, catching panics and propagating them to ensure
/// that the container reliably shuts down.
///
/// Prints log output from the container in the event an error occurred.
fn docker_run<A, S, F>(args: A, f: F)
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    F: FnOnce() -> () + panic::UnwindSafe,
{
    let container_id = exec_docker_command("run", args);
    let result = panic::catch_unwind(f);

    if result.is_err() {
        let logs = exec_docker_command("logs", &[&container_id]);

        println!("\n---- docker stdout ----");
        println!("{}", logs);
    }

    exec_docker_command("kill", &[&container_id]);

    if let Err(err) = result {
        panic::resume_unwind(err);
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
