/// GenesisState defines the raw genesis transaction in JSON.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// gen_txs defines the genesis transactions.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub gen_txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
