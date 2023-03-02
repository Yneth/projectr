use futures::stream::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::Client;
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, UpdateModifications};
use mongodb::results::{InsertOneResult};
use serde_json::Value;

const MONGO_DB: &str = "test_db";
const MONGO_COLL: &str = "test_coll";

pub async fn increase_counter(mongo: Client) -> anyhow::Result<i32> {
    tracing::trace!("increase counter");

    let collection = mongo
        .database(MONGO_DB)
        .collection::<Document>("counter");

    let result = collection
        .find_one_and_update(
            doc! { "_id": "view_counter" },
            UpdateModifications::Document(doc! { "$inc": { "value": 1 } }),
            FindOneAndUpdateOptions::builder()
                .upsert(true)
                .build())
        .await;

    dbg!(&result);

    tracing::trace!("increase counter result - {:?}", result);

    result
        .map_err(anyhow::Error::msg)
        .map(|maybe_doc| maybe_doc
            .map(|doc| doc.get_i32("value"))
            .map(|result| result.ok())
            .flatten()
            .unwrap_or(0))
}

pub async fn insert_data(mongo: Client, id: String, data: String) -> anyhow::Result<InsertOneResult> {
    mongo.database(MONGO_DB)
        .collection(MONGO_COLL)
        .insert_one(doc! {"_id": id, "data": data}, None)
        .await
        .map_err(anyhow::Error::msg)
}

pub async fn read_data(mongo: Client) -> anyhow::Result<Value> {
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