use {
    crate::api::ChainId,
    prometheus_client::{
        encoding::EncodeLabelSet,
        metrics::{
            counter::Counter,
            family::Family,
            gauge::Gauge,
        },
        registry::Registry,
    },
    std::sync::atomic::AtomicU64,
    tokio::sync::RwLock,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct RequestLabel {
    pub value: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct RpcLabel {
    pub chain_id: ChainId,
    pub uri:      String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct AccountLabel {
    pub chain_id: String,
    pub address:  String,
}

pub struct Metrics {
    pub registry: RwLock<Registry>,

    pub request_counter: Family<RequestLabel, Counter>,

    pub current_sequence_number: Family<AccountLabel, Gauge>,
    pub end_sequence_number:     Family<AccountLabel, Gauge>,
    pub balance:                 Family<AccountLabel, Gauge<f64, AtomicU64>>,
    pub collected_fee:           Family<AccountLabel, Gauge<f64, AtomicU64>>,
    pub total_gas_spent:         Family<AccountLabel, Gauge<f64, AtomicU64>>,
    pub requests:                Family<AccountLabel, Counter>,
    pub requests_processed:      Family<AccountLabel, Counter>,
    pub reveals:                 Family<AccountLabel, Counter>,
}

impl Metrics {
    pub fn new() -> Self {
        let mut metrics_registry = Registry::default();

        let http_requests = Family::<RequestLabel, Counter>::default();
        metrics_registry.register(
            // With the metric name.
            "http_requests",
            // And the metric help text.
            "Number of HTTP requests received",
            http_requests.clone(),
        );

        let current_sequence_number = Family::<AccountLabel, Gauge>::default();
        metrics_registry.register(
            // With the metric name.
            "current_sequence_number",
            // And the metric help text.
            "The sequence number for a new request.",
            current_sequence_number.clone(),
        );

        let end_sequence_number = Family::<AccountLabel, Gauge>::default();
        metrics_registry.register(
            // With the metric name.
            "end_sequence_number",
            // And the metric help text.
            "The sequence number for the end request.",
            end_sequence_number.clone(),
        );

        let requests = Family::<AccountLabel, Counter>::default();
        metrics_registry.register(
            // With the metric name.
            "requests",
            // And the metric help text.
            "Number of requests received through events",
            requests.clone(),
        );

        let requests_processed = Family::<AccountLabel, Counter>::default();
        metrics_registry.register(
            // With the metric name.
            "requests_processed",
            // And the metric help text.
            "Number of requests processed",
            requests_processed.clone(),
        );

        let reveals = Family::<AccountLabel, Counter>::default();
        metrics_registry.register(
            // With the metric name.
            "reveal",
            // And the metric help text.
            "Number of reveals",
            reveals.clone(),
        );

        let balance = Family::<AccountLabel, Gauge<f64, AtomicU64>>::default();
        metrics_registry.register(
            // With the metric name.
            "balance",
            // And the metric help text.
            "Balance of the keeper",
            balance.clone(),
        );

        let collected_fee = Family::<AccountLabel, Gauge<f64, AtomicU64>>::default();
        metrics_registry.register(
            // With the metric name.
            "collected_fee",
            // And the metric help text.
            "Collected fee on the contract",
            collected_fee.clone(),
        );

        let total_gas_spent = Family::<AccountLabel, Gauge<f64, AtomicU64>>::default();
        metrics_registry.register(
            // With the metric name.
            "total_gas_spent",
            // And the metric help text.
            "Total gas spent revealing requests",
            total_gas_spent.clone(),
        );

        Metrics {
            registry: RwLock::new(metrics_registry),
            request_counter: http_requests,
            current_sequence_number,
            end_sequence_number,
            requests,
            requests_processed,
            reveals,
            balance,
            collected_fee,
            total_gas_spent,
        }
    }
}