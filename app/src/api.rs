use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Html, IntoResponse, Response};
use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{AppContext, elastic, mongo};

pub struct AppError(anyhow::Error);

#[derive(Serialize)]
pub struct RootResponse {
    #[serde(with = "serde_millis")]
    time: SystemTime,
    config: Value,
}

#[derive(Serialize)]
pub struct InsertResponse {
    #[serde(with = "serde_millis")]
    time: SystemTime,
}

#[derive(Serialize)]
pub struct ReadResponse {
    #[serde(with = "serde_millis")]
    time: SystemTime,
    data: Value,
}

#[axum_macros::debug_handler]
pub async fn root(
    State(context): State<AppContext>
) -> (StatusCode, Json<RootResponse>) {
    (StatusCode::OK, Json(RootResponse {
        time: SystemTime::now(),
        config: serde_json::to_value(&context.args)
            .expect("should convert args to json"),
    }))
}

#[axum_macros::debug_handler]
pub async fn index(
    State(context): State<AppContext>
) -> Result<Response, AppError> {
    tracing::trace!("serving index");

    let counter = mongo::increase_counter(context.mongo.clone()).await?;
    let html = format!(
        r#"<html><head><title>index</title><head><body><pre>view count: {}
time: {}
mongo: {}
elastic: {}
</pre></body></html>"#,
        counter,
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        mongo::read_data(context.mongo.clone()).await?,
        elastic::read_data(context.elasticsearch.clone()).await?
    );
    Ok((StatusCode::OK, Html(html)).into_response())
}

#[axum_macros::debug_handler]
pub async fn insert_elastic(State(context): State<AppContext>) -> (StatusCode, Json<InsertResponse>) {
    let id = Uuid::new_v4().to_string();
    let data = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let mut status_code = StatusCode::CREATED;
    if let Err(e) = elastic::insert_data(context.elasticsearch.clone(), id.to_string(), data.to_string()).await {
        tracing::error!("failed to read elastic {:?}", e);
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
    }

    (status_code, Json(InsertResponse {
        time: SystemTime::now()
    }))
}

#[axum_macros::debug_handler]
pub async fn read_elastic(State(context): State<AppContext>) -> (StatusCode, Json<ReadResponse>) {
    let mut data = Value::Null;
    let mut status_code = StatusCode::CREATED;

    match elastic::read_data(context.elasticsearch.clone()).await {
        Err(e) => {
            tracing::error!("failed to read elastic {:?}", e);
            status_code = StatusCode::INTERNAL_SERVER_ERROR;
        }
        Ok(result) => data = result,
    }

    (status_code, Json(ReadResponse {
        time: SystemTime::now(),
        data,
    }))
}

#[axum_macros::debug_handler]
pub async fn insert_mongo(State(context): State<AppContext>) -> (StatusCode, Json<InsertResponse>) {
    let id = Uuid::new_v4().to_string();
    let data = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let mut status_code = StatusCode::CREATED;

    if let Err(e) = mongo::insert_data(context.mongo.clone(), id.to_string(), data.to_string()).await {
        tracing::error!("failed to read elastic {:?}", e);
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
    }

    (status_code, Json(InsertResponse {
        time: SystemTime::now()
    }))
}

#[axum_macros::debug_handler]
pub async fn read_mongo(State(context): State<AppContext>) -> (StatusCode, Json<ReadResponse>) {
    let mut response_data = Value::Null;
    let mut status_code = StatusCode::CREATED;

    match mongo::read_data(context.mongo.clone()).await {
        Err(e) => {
            tracing::error!("failed to read mongo {:?}", e);
            status_code = StatusCode::INTERNAL_SERVER_ERROR;
        }
        Ok(result) => response_data = result,
    }

    (status_code, Json(ReadResponse {
        time: SystemTime::now(),
        data: response_data,
    }))
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}


impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error> {
    fn from(e: E) -> Self {
        Self(e.into())
    }
}