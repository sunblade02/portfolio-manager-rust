use chrono::{Local, Utc};
use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use tokio::{sync::Mutex, time::{interval, Duration}};
use tracing::{Event, Subscriber, field::{Field, Visit}};
use tracing_subscriber::{Layer, EnvFilter, prelude::*};
use redis::{AsyncCommands, aio::MultiplexedConnection, Client};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc, error::Error};

#[derive(Serialize, Deserialize)]
struct LogEntry {
    level: String,
    timestamp: String,
    action: Option<String>,
    message: Option<String>,
    user_id: Option<String>,
    extra: Option<String>
}

struct LogVisitor {
    action: Option<String>,
    message: Option<String>,
    user_id: Option<String>,
    extra: Option<String>
}

impl Visit for LogVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "action" => self.action = Some(value.to_string()),
            "message" => self.message = Some(value.to_string()),
            "user_id" => self.user_id = Some(value.to_string()),
            "extra" => self.extra = Some(value.to_string()),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "action" => self.action = Some(format!("{:?}", value)),
            "message" => self.message = Some(format!("{:?}", value)),
            "user_id" => self.user_id = Some(format!("{:?}", value)),
            "extra" => self.user_id = Some(format!("{:?}", value)),
            _ => {}
        }
    }
}

struct RedisLayer {
    redis: Arc<Mutex<MultiplexedConnection>>,
}

impl<S: Subscriber> Layer<S> for RedisLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut visitor = LogVisitor { action: None, message: None, user_id: None, extra: None };
        event.record(&mut visitor);
        
        let metadata = event.metadata();
        let log = LogEntry {
            level: metadata.level().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            action: visitor.action,
            message: visitor.message,
            user_id: visitor.user_id,
            extra: visitor.extra,
        };

        let redis = self.redis.clone();
        tokio::spawn(async move {
            if let Ok(json) = serde_json::to_string(&log) {
                let mut conn = redis.lock().await;
                let _: Result<(), _> = conn.rpush("logs", json).await;
            }
        });
    }
}

pub async fn get_redis_conn() -> Result<MultiplexedConnection, Box<dyn Error>> {
    let redis_url =  env::var("REDIS_URL")
        .map_err(|_| "REDIS_URL must be set")?;

    let client = Client::open(redis_url)?;
    let conn = client.get_multiplexed_async_connection().await?;

    Ok(conn)
}

pub async fn setup_logging() -> Result<(), Box<dyn Error>> {
    let redis_conn = get_redis_conn().await?;
    let redis_layer = RedisLayer { redis: Arc::new(Mutex::new(redis_conn)) };

    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(redis_layer)
        .init();

    Ok(())
}

pub async fn start_log_flusher() {
    let mongodb_url =  env::var("MONGODB_URL")
        .expect("MONGODB_URL must be set");

    let mongo_client = MongoClient::with_options(
        ClientOptions::parse(mongodb_url).await.expect("Failed to parse MONGODB_URL")
    ).expect("Failed to create Mongo client");

    let collection: mongodb::Collection<LogEntry> = mongo_client.database("portfolio_manager").collection("logs");

    let mut redis_conn = get_redis_conn().await.expect("Failed to connect to Redis");

    let mut ticker = interval(Duration::from_secs(60));

    println!("Worker started. Waiting for logs...");

    loop {
        ticker.tick().await;

        // Get logs from Redis database
        let logs: Vec<String> = redis_conn.lrange("logs", 0, -1).await.unwrap_or_default();
        if logs.is_empty() {
            continue;
        }

        // Remove logs from Redis database
        if let Err(e) = redis_conn.del::<_, ()>("logs").await {
            eprintln!("Error deleting from Redis: {:?}", e);
        }

        let mut docs = Vec::new();
        for log in logs {
            match serde_json::from_str::<LogEntry>(&log) {
                Ok(entry) => docs.push(entry),
                Err(e) => {
                    eprintln!("Error parsing JSON log: {:?}", e);
                    continue;
                }
            }
        }

        if docs.is_empty() {
            continue;
        }

        // Insert logs into MongoDB database
        if let Err(e) = collection.insert_many(&docs, None).await {
            eprintln!("Error from Mongo: {:?}", e);
        }

        let now = Local::now(); // ou Utc::now() pour UTC
        let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("{} : {} log(s) inserted into MongoDB.", formatted, docs.len());
    }
}