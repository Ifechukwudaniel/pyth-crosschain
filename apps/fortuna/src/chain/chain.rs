use {
    anyhow::{
        Error,
        Result,
    },
    axum::async_trait,
};

pub type ChainBlockNumber = u64;

#[derive(Clone, Debug)]
pub struct RequestWithCallbackData {
    /// The sequence number of the request.
    pub sequence_number:    u64,
    /// The random number submitted by the user while requesting a callback.
    pub user_random_number: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct ProviderInfo {
    pub accrued_fee:             f64,
    pub current_sequence_number: u64,
    pub end_sequence_number:     u64,
}

#[async_trait]
pub trait ChainReader: Send + Sync {
    /// Returns data of all the requests with callback made on chain between
    /// the given block numbers.
    async fn get_requests_with_callback_data(
        &self,
        from_block: ChainBlockNumber,
        to_block: ChainBlockNumber,
    ) -> Result<Vec<RequestWithCallbackData>>;

    /// Returns the latest block which is included into the chain and
    /// is safe from reorgs.
    async fn get_latest_safe_block(&self) -> Result<ChainBlockNumber>;

    /// Returns the provider address for which we are fulfilling requests.
    fn get_provider_address(&self) -> String;

    /// Returns the provider info for which we are fulfilling requests.
    async fn get_provider_info(&self) -> Result<ProviderInfo>;
}

pub enum RevealError {
    GasLimitExceeded,
    ContractError(Error),
    RpcError(Error),
    Unknown(Error),
}

pub struct RevealSuccess {
    pub tx_hash:  String,
    pub gas_used: f64,
}

#[async_trait]
pub trait ChainWriter: ChainReader {
    /// Fulfill the given request on chain with the given provider revelation.
    async fn reveal_with_callback(
        &self,
        request_with_callback_data: RequestWithCallbackData,
        provider_revelation: [u8; 32],
    ) -> Result<RevealSuccess, RevealError>;

    fn get_writer_address(&self) -> String;

    async fn get_writer_balance(&self) -> Result<f64>;
}
