use std::net::SocketAddr;
use std::time::{Duration, SystemTime};

use axum::{
    http::StatusCode,
    Json,
    Router, routing::{get, post},
};
use axum::extract::State;
use elasticsearch::{CreateParts, Elasticsearch, Error, SearchParts, UpdateParts};
use elasticsearch::http::transport::Transport;
use mongodb::bson::{doc, Document, Uuid};
use mongodb::{Client};
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::results::InsertOneResult;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use futures::stream::StreamExt;

const DEFAULT_PORT: u32 = 5000;

const MONGO_DB: &str = "test_db";
const MONGO_COLL: &str = "test_coll";
const ES_INDEX: &str = "test_index";

#[derive(Clone, Debug, Serialize)]
struct Args {
    port: u16,
    mongodb_url: String,
    elasticsearch_url: String,
}

#[derive(Debug, Clone)]
struct AppContext {
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
        .route("/", get(root))
        .route("/insert", post(insert))
        .route("/read", get(read))
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

#[axum_macros::debug_handler]
async fn root(
    State(context): State<AppContext>
) -> (StatusCode, Json<RootResponse>) {
    (StatusCode::OK, Json(RootResponse {
        time: SystemTime::now(),
        config: serde_json::to_value(&context.args)
            .expect("should convert args to json"),
    }))
}

async fn insert_mongo(mongo: Client, id: String, data: String) -> anyhow::Result<InsertOneResult> {
    mongo.database(MONGO_DB)
        .collection(MONGO_COLL)
        .insert_one(doc! {"_id": id, "data": data}, None)
        .await
        .map_err(anyhow::Error::msg)
}

async fn insert_es(elastic: Elasticsearch, id: String, data: String) -> anyhow::Result<Value> {
    let response = elastic.create(CreateParts::IndexId(ES_INDEX, &id))
        .body(json!({ "data": data }))
        .send()
        .await
        .map_err(anyhow::Error::msg)?;

    let response_body = response.text().await?;

    serde_json::to_value(response_body)
        .map_err(anyhow::Error::msg)
}

#[axum_macros::debug_handler]
async fn insert(State(context): State<AppContext>) -> (StatusCode, Json<InsertResponse>) {
    let response = InsertResponse {
        time: SystemTime::now()
    };

    let id = Uuid::new().to_string();
    let data = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let mongo_fut = tokio::spawn(
        insert_mongo(context.mongo.clone(), id.to_owned(), data.to_owned()));

    let elastic_fut = tokio::spawn(
        insert_es(context.elasticsearch.clone(), id.to_string(), data.to_string()));

    let mut status_code = StatusCode::CREATED;
    let (mongo_res, elastic_res) = tokio::join!(mongo_fut, elastic_fut);

    if matches!(mongo_res, Err(_) | Ok(Err(_))) {
        tracing::error!("failed to insert mongo {:?}", mongo_res);
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
    }
    if matches!(elastic_res, Err(_) | Ok(Err(_))) {
        tracing::error!("failed to insert elastic {:?}", elastic_res);
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
    }

    (status_code, Json(response))
}

async fn read_mongo(mongo: Client) -> anyhow::Result<Value> {
    let result: Vec<Document> = mongo.database(MONGO_DB)
        .collection::<Document>(MONGO_COLL)
        .find(doc! {}, Some(FindOptions::builder().sort(doc! {"$natural": -1}).limit(10).build()))
        .await
        .map_err(anyhow::Error::msg)?
        .collect::<Vec<Result<Document, mongodb::error::Error>>>()
        .await
        .into_iter()
        .filter_map(|res| res.ok())
        .collect();

    serde_json::to_value(result)
        .map_err(anyhow::Error::msg)
}

async fn read_es(elastic: Elasticsearch) -> anyhow::Result<Value> {
    let response = elastic.search(SearchParts::Index(&vec![ES_INDEX]))
        .send()
        .await
        .map_err(anyhow::Error::msg)?;

    let response_body = response.text().await?;
    serde_json::to_value(response_body)
        .map_err(anyhow::Error::msg)
}

#[axum_macros::debug_handler]
async fn read(State(context): State<AppContext>) -> (StatusCode, Json<ReadResponse>) {
    let mongo_fut = tokio::spawn(read_mongo(context.mongo.clone()));
    let elastic_fut = tokio::spawn(read_es(context.elasticsearch.clone()));

    let mut mongo = Value::Null;
    let mut es = Value::Null;
    let mut status_code = StatusCode::CREATED;

    let (mongo_res, elastic_res) = tokio::join!(mongo_fut, elastic_fut);

    if matches!(mongo_res, Err(_) | Ok(Err(_))) {
        tracing::error!("failed to read mongo {:?}", mongo_res);
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
    }
    if let Ok(Ok(r)) = mongo_res {
        mongo = r;
    }

    if matches!(elastic_res, Err(_) | Ok(Err(_))) {
        tracing::error!("failed to read elastic {:?}", elastic_res);
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
    }
    if let Ok(Ok(r)) = elastic_res {
        es = r;
    }

    (status_code, Json(ReadResponse {
        time: SystemTime::now(),
        mongo,
        es
    }))
}

#[derive(Serialize)]
struct RootResponse {
    #[serde(with = "serde_millis")]
    time: SystemTime,
    config: Value,
}

#[derive(Serialize)]
struct InsertResponse {
    #[serde(with = "serde_millis")]
    time: SystemTime,
}

#[derive(Serialize)]
struct ReadResponse {
    #[serde(with = "serde_millis")]
    time: SystemTime,
    mongo: Value,
    es: Value,
}