#![allow(clippy::just_underscores_and_digits)]
#![feature(slice_flatten)]

use std::error::Error;

use clap::Parser;

use {
    anyhow::Result,
    axum::{
        extract::Extension,
        Router,
        routing::get,
    },
    crate::config::RunOptions,
    serde_qs::axum::QsQueryConfig,
    std::sync::{
        Arc,
        atomic::Ordering,
    },
    tokio::{
        signal,
        sync::mpsc::Receiver,
    },
    tower_http::cors::CorsLayer,
    utoipa::OpenApi,
    utoipa_swagger_ui::SwaggerUi,
};
use ethereum::register_provider;
use ethereum::request_randomness;
use ethereum::get_request;

use crate::api::{ApiState, get_random_value};
use crate::state::PebbleHashChain;
use crate::ethereum::provider;

pub mod api;
pub mod config;
pub mod ethereum;
pub mod state;

const SECRET: [u8; 32] = [0u8; 32];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match config::Options::parse() {
        config::Options::GetRequest(opts) => get_request(&opts).await,
        config::Options::Run(opts) => run(&opts).await,
        config::Options::RegisterProvider(opts) => register_provider(&opts).await,
        config::Options::RequestRandomness(opts) => request_randomness(&opts).await,
    }
}

async fn run(opts: &RunOptions) -> Result<(), Box<dyn Error>> {
    #[derive(OpenApi)]
    #[openapi(
    paths(
      api::get_random_value,
    ),
    components(
    schemas(
      api::GetRandomValueResponse
    )
    ),
    tags(
    (name = "hermes", description = "Pyth Real-Time Pricing API")
    )
    )]
    struct ApiDoc;

    let random = [0u8; 32]; // rand::random::<[u8; 32]>();
    let mut chain = PebbleHashChain::from_config(&opts.randomness, random)?;
    let contract = Arc::new(provider(&opts.ethereum).await?);
    let provider = opts.provider_address.parse::<Address>()?;
    let mut state = ApiState{ state: Arc::new(chain), provider: contract, provider_address: provider};

    // Initialize Axum Router. Note the type here is a `Router<State>` due to the use of the
    // `with_state` method which replaces `Body` with `State` in the type signature.
    let app = Router::new();
    let app = app
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/api/get_random_value", get(get_random_value))
        .with_state(state.clone())
        // Permissive CORS layer to allow all origins
        .layer(CorsLayer::permissive());

    // Binds the axum's server to the configured address and port. This is a blocking call and will
    // not return until the server is shutdown.
    axum::Server::try_bind(&opts.addr)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}