use std::env;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgListener, PgPoolOptions};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "event_id")]
    id: u64,
    #[serde(rename = "event_name")]
    name: String,
    #[serde(rename = "event_datetime")]
    datetime: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "operation")]
pub enum Notification {
    Insert {
        timestamp: DateTime<Utc>,
        schema: String,
        table: String,
        new: Event,
    },
    Update {
        timestamp: DateTime<Utc>,
        schema: String,
        table: String,
        old: Event,
        new: Event,
    },
    Delete {
        timestamp: DateTime<Utc>,
        schema: String,
        table: String,
        old: Event,
    },
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Loaded environment variables from .env");

    let database_url =
        env::var("DATABASE_URL").expect("Loaded database url from environmental variables.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Established connection to database pool.");

    let mut listener = PgListener::connect_with(&pool)
        .await
        .expect("Initialised Postgres listener.");

    listener
        .listen("event")
        .await
        .expect("Listening to channel.");

    loop {
        let notification = listener.recv().await.expect("Received notification.");
        let payload = notification.payload();
        let notification: Notification =
            serde_json::from_str(payload).expect("Deserialised notification");
        dbg!(notification);
    }
}
