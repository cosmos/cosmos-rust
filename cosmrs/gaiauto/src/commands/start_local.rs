//! `start` subcommand - example of how to write a subcommand

use crate::config::GaiautoConfig;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;

/// This is added by Ugochi
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

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct StartLocalCmd {
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

impl Runnable for StartLocalCmd {
    /// Start the application.
    fn run(&self) {
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
            dev::GAIA_DOCKER_IMAGE,
            CHAIN_ID,
            &sender_account_id.to_string(),
        ];

        dev::docker_run(&docker_args, || {
            abscissa_tokio::run(&APP, async {
                let rpc_address = format!("http://localhost:{}", RPC_PORT);
                let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();
                dev::poll_for_first_block(&rpc_client).await;

                let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();

                if tx_commit_response.check_tx.code.is_err() {
                    panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
                }

                if tx_commit_response.deliver_tx.code.is_err() {
                    panic!("deliver_tx failed: {:?}", tx_commit_response.deliver_tx);
                }

                let tx = dev::poll_for_tx(&rpc_client, tx_commit_response.hash).await;
                assert_eq!(&tx_body, &tx.body);
                assert_eq!(&auth_info, &tx.auth_info);
            })
            .unwrap_or_else(|e| {
                status_err!("executor exited with error: {}", e);
                std::process::exit(1);
            });
        });
    }
}

impl config::Override<GaiautoConfig> for StartLocalCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: GaiautoConfig) -> Result<GaiautoConfig, FrameworkError> {
        Ok(config)
    }
}
