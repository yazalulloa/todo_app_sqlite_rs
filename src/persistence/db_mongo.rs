use std::fmt::Error;

use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};

pub async fn load_database() -> Result<Database, Error> {
    let url = std::env::var("MONGO_URL")
        .ok()
        .expect("mongo url is not set");
    let options = ClientOptions::parse(url).await.expect("failed to parse");
    let client = Client::with_options(options).expect("Couldn't connect to Mongo");
    Ok(client.default_database().expect("database"))
}

pub async fn collection(col_name: &str) -> Collection<Document> {
    load_database().await.expect("").collection(col_name)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: u16,
    pub title: String,
    pub completed: bool,
}
