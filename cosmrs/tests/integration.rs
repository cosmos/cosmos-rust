//! Integration test which submits transactions to a local `gaia` node.
//!
//! Requires Docker.

#![cfg(feature = "dev")]

use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1,
    dev, rpc,
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo},
    Coin,
};
use std::{panic, str};

/// Chain ID to use for tests
const CHAIN_ID: &str = "cosmrs-test";

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
    .to_any()
    .unwrap();

    let chain_id = CHAIN_ID.parse().unwrap();
    let sequence_number = 0;
    let gas = 100_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    let tx_body = tx::BodyBuilder::new().msg(msg_send).memo(MEMO).finish();
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, ACCOUNT_NUMBER).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

    let docker_args = [
        "-d",
        "-p",
        &format!("{}:{}", RPC_PORT, RPC_PORT),
        dev::GAIA_DOCKER_IMAGE,
        CHAIN_ID,
        &sender_account_id.to_string(),
    ];

    dev::docker_run(&docker_args, || {
        init_tokio_runtime().block_on(async {
            let rpc_address = format!("http://localhost:{}", RPC_PORT);
            let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();
            dev::poll_for_first_block(&rpc_client).await;

            let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();

            if tx_commit_response.check_tx.code.is_err() {
                panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
            }

            if tx_commit_response.tx_result.code.is_err() {
                panic!("tx_result error: {:?}", tx_commit_response.tx_result);
            }

            let tx = dev::poll_for_tx(&rpc_client, tx_commit_response.hash).await;
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
