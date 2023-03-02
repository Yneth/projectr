mod mongo;
mod elastic;
mod api;

use std::net::SocketAddr;
use std::time::{Duration};

use axum::{
    Router, routing::{get, post},
};
use elasticsearch::{Elasticsearch};
use elasticsearch::http::transport::Transport;
use mongodb::{Client};
use mongodb::options::{ClientOptions};
use serde::Serialize;


const DEFAULT_PORT: u32 = 5000;

#[derive(Clone, Debug, Serialize)]
pub struct Args {
    port: u16,
    mongodb_url: String,
    elasticsearch_url: String,
}

#[derive(Debug, Clone)]
pub struct AppContext {
    pub args: Args,
    pub mongo: Client,
    pub elasticsearch: Elasticsearch,
}

async fn build_args() -> anyhow::Result<Args> {
    Ok(Args {
        port: std::env::var("PORT").unwrap_or(DEFAULT_PORT.to_string()).parse()?,
        mongodb_url: std::env::var("MONGODB_URL").expect("mongodb is required"),
        elasticsearch_url: std::env::var("ELASTICSEARCH_URL").expect("elasticsearch is required"),
    })
}

async fn build_context(args: Args) -> anyhow::Result<AppContext> {
    let mut mongodb_client_opts = ClientOptions::parse(&args.mongodb_url).await?;
    mongodb_client_opts.connect_timeout = Some(Duration::from_secs(5));
    mongodb_client_opts.server_selection_timeout = Some(Duration::from_secs(5));

    let transport = Transport::single_node(&args.elasticsearch_url)?;
    let es_client = Elasticsearch::new(transport);

    Ok(AppContext {
        args,
        mongo: Client::with_options(mongodb_client_opts)?,
        elasticsearch: es_client,
    })
}

async fn do_main() -> anyhow::Result<()> {
    let args: Args = build_args().await?;
    tracing::info!("parsed args: {:?}", args);

    let context: AppContext = build_context(args).await?;
    tracing::info!("context: {:?}", context);

    let addr = SocketAddr::from(([0, 0, 0, 0], context.args.port.clone()));

    let app = Router::new()
        .route("/", get(api::root))
        .route("/index", get(api::index))
        .route("/insert_elastic", post(api::insert_elastic))
        .route("/read_elastic", get(api::read_elastic))
        .route("/insert_mongo", post(api::insert_mongo))
        .route("/read_mongo", get(api::read_mongo))
        .with_state(context);

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if let Err(e) = do_main().await {
        tracing::error!("application failed reason: {}", e);
    }
}