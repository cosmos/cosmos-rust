//! Raw transaction.

use crate::{
    proto::{
        self,
        traits::{Message, MessageExt},
    },
    Result,
};

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
        Ok(Raw(Message::decode(bytes)?))
    }

    /// Serialize raw transaction as a byte vector.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.0.to_bytes()?)
    }

    /// Broadcast this transaction using the provided RPC client
    #[cfg(feature = "rpc")]
    pub async fn broadcast_commit<C>(&self, client: &C) -> Result<TxCommitResponse>
    where
        C: rpc::Client + Send + Sync,
    {
        Ok(client.broadcast_tx_commit(self.to_bytes()?).await?)
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
