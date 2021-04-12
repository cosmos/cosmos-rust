//! Raw transaction.

use crate::{prost_ext::MessageExt, proto, Result};

#[cfg(feature = "rpc")]
use crate::rpc;

/// Response from `/broadcast_tx_commit`
#[cfg(feature = "rpc")]
pub type TxCommitResponse = rpc::endpoint::broadcast::tx_commit::Response;

/// Raw transaction
#[derive(Clone, Debug)]
pub struct Raw(proto::cosmos::tx::v1beta1::TxRaw);

impl Raw {
    /// Deserialize raw transaction from serialized protobuf.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Raw(prost::Message::decode(bytes)?))
    }

    /// Serialize raw transaction as a byte vector.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.0.to_bytes()
    }

    /// Broadcast this transaction using the provided RPC client
    #[cfg(feature = "rpc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rpc")))]
    pub async fn broadcast_commit<C>(&self, client: &C) -> Result<TxCommitResponse>
    where
        C: rpc::Client + Send + Sync,
    {
        Ok(client.broadcast_tx_commit(self.to_bytes()?.into()).await?)
    }
}

impl From<proto::cosmos::tx::v1beta1::TxRaw> for Raw {
    fn from(tx: proto::cosmos::tx::v1beta1::TxRaw) -> Self {
        Raw(tx)
    }
}

impl From<Raw> for proto::cosmos::tx::v1beta1::TxRaw {
    fn from(tx: Raw) -> proto::cosmos::tx::v1beta1::TxRaw {
        tx.0
    }
}
