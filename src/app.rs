use std::sync::{Arc, Mutex};

use oauth2::basic::BasicClient;
use redis::aio::MultiplexedConnection;
use redis_work_queue::KeyPrefix;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: Arc<Mutex<MultiplexedConnection>>,
    pub work_queue_key: KeyPrefix,
    pub google_oauth: BasicClient,
    pub google_userinfo_url: String,
    pub csh_oauth: BasicClient,
    pub csh_userinfo_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleRiderChange {
    pub event_id: i32,
    pub car_id: i32,
    pub rider_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MultipleRiderChange {
    pub event_id: i32,
    pub car_id: i32,
    pub old_riders: Vec<String>,
    pub new_riders: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RedisJob {
    Join(SimpleRiderChange),
    Leave(SimpleRiderChange),
    RiderUpdate(MultipleRiderChange),
}
