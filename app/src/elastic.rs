use std::time::{SystemTime, UNIX_EPOCH};

use elasticsearch::{CreateParts, Elasticsearch, SearchParts};
use serde_json::{json, Value};

const ES_INDEX: &str = "test_index";

pub async fn insert_data(elastic: Elasticsearch, id: String, data: String) -> anyhow::Result<Value> {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let response = elastic.create(CreateParts::IndexId(ES_INDEX, &id))
        .body(json!({ "data": data, "time": time }))
        .send()
        .await
        .map_err(anyhow::Error::msg)?;

    let response_body = response.text().await?;

    serde_json::to_value(response_body)
        .map_err(anyhow::Error::msg)
}

pub async fn read_data(elastic: Elasticsearch) -> anyhow::Result<Value> {
    let response = elastic.search(SearchParts::Index(&vec![ES_INDEX]))
        .body(json!({ "sort": [ { "time": { "order": "desc" } } ] }))
        .send()
        .await
        .map_err(anyhow::Error::msg)?;

    let response_body = response.text().await?;
    serde_json::to_value(response_body)
        .map_err(anyhow::Error::msg)
}
