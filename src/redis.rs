use crate::app::RedisJob;
use anyhow::{anyhow, Result};
use redis::aio::MultiplexedConnection;
use redis_work_queue::{Item, WorkQueue};

pub struct RedisQueue {
    pub redis: MultiplexedConnection,
    pub work_queue: WorkQueue,
}

impl RedisQueue {
    pub async fn insert_job(&mut self, job: RedisJob) -> Result<()> {
        let item = Item::from_json_data(&job)?;
        match self.work_queue.add_item(&mut self.redis, &item).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(anyhow!("Job ID Already Exists in Queue")),
            Err(err) => Err(anyhow!("{}", err)),
        }
    }
}
