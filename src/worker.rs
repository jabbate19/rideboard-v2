use std::{
    collections::{HashMap, HashSet},
    env,
};

use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
use redis_work_queue::{Item, KeyPrefix, WorkQueue};
use sqlx::{postgres::PgPoolOptions, query, query_as, Pool, Postgres};
use std::time::Duration;

use crate::{
    app::{RedisJob, SimpleRiderChange, UserData},
    pings::PingClient,
};

struct RedisError {
    pub msg: String,
    pub should_retry: bool,
}

pub async fn main() -> std::io::Result<()> {
    let db = redis::Client::open(env::var("REDIS_URL").expect("REDIS_URL must be set"))
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let work_queue = WorkQueue::new(KeyPrefix::from("rideboard"));

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool");

    let pings = PingClient::new(
        env::var("PINGS_TOKEN").expect("PINGS_TOKEN must be set"),
        env::var("PINGS_JOIN_ROUTE").expect("PINGS_JOIN_ROUTE must be set"),
        env::var("PINGS_LEAVE_ROUTE").expect("PINGS_LEAVE_ROUTE must be set"),
        env::var("PINGS_ADD_ROUTE").expect("PINGS_ADD_ROUTE must be set"),
        env::var("PINGS_REMOVE_ROUTE").expect("PINGS_REMOVE_ROUTE must be set"),
    )
    .unwrap();

    work_loop(db, work_queue, db_pool, pings).await.unwrap();
    Ok(())
}

async fn get_event_name(event_id: i32, db_pool: &Pool<Postgres>) -> String {
    query!(r#"SELECT name FROM event WHERE id = $1"#, event_id)
        .fetch_one(db_pool)
        .await
        .unwrap()
        .name
}

async fn get_driver(car_id: i32, db_pool: &Pool<Postgres>) -> UserData {
    query_as!(
        UserData,
        r#"
        SELECT users.id AS "id!", users.realm::text AS "realm!", users.name AS "name!", users.email AS "email!"
        FROM car JOIN users ON car.driver = users.id WHERE car.id = $1;
        "#, car_id
    ).fetch_one(db_pool).await.unwrap()
}

async fn get_users_by_id(ids: Vec<String>, db_pool: &Pool<Postgres>) -> HashMap<String, UserData> {
    let data = query_as!(
        UserData,
        r#"
        SELECT id AS "id!", realm::text AS "realm!", name AS "name!", email AS "email!"
        FROM users WHERE id IN (SELECT UNNEST($1::VARCHAR[]))
        "#,
        &ids
    )
    .fetch_all(db_pool)
    .await
    .unwrap();
    HashMap::from_iter(data.iter().map(|user| (user.id.clone(), user.clone())))
}

async fn get_simple_data(
    data: SimpleRiderChange,
    db_pool: &Pool<Postgres>,
) -> (String, UserData, UserData) {
    let rider = query_as!(
        UserData,
        r#"
        SELECT users.id AS "id!", users.realm::text AS "realm!", users.name AS "name!", users.email AS "email!"
        FROM users where id = $1;
        "#, data.rider_id
    ).fetch_one(db_pool).await.unwrap();
    (
        get_event_name(data.event_id, db_pool).await,
        get_driver(data.car_id, db_pool).await,
        rider,
    )
}

async fn work(job: &Item, db_pool: &Pool<Postgres>, pings: &PingClient) -> Result<(), RedisError> {
    let job_data: RedisJob = job.data_json().map_err(|_err| RedisError {
        msg: "Failed to Parse into Job".to_string(),
        should_retry: false,
    })?;
    match job_data {
        RedisJob::Join(data) => {
            let (event_name, driver, rider) = get_simple_data(data, db_pool).await;
            if driver.realm != "csh" {
                return Ok(());
            }
            pings
                .send_join(
                    &driver.email.trim_end_matches("@csh.rit.edu"),
                    &rider.name,
                    &event_name,
                )
                .await
                .unwrap();
        }
        RedisJob::Leave(data) => {
            let (event_name, driver, rider) = get_simple_data(data, db_pool).await;
            if driver.realm != "csh" {
                return Ok(());
            }
            pings
                .send_leave(
                    &driver.email.trim_end_matches("@csh.rit.edu"),
                    &rider.name,
                    &event_name,
                )
                .await
                .unwrap();
        }
        RedisJob::RiderUpdate(data) => {
            let event_name = get_event_name(data.event_id, db_pool).await;
            let driver = get_driver(data.car_id, db_pool).await;
            let old_set: HashSet<String> = HashSet::from_iter(data.old_riders);
            let new_set: HashSet<String> = HashSet::from_iter(data.new_riders);
            let user_map = get_users_by_id(
                old_set
                    .difference(&new_set)
                    .chain(new_set.difference(&old_set))
                    .map(|s| s.to_string())
                    .collect(),
                db_pool,
            )
            .await;
            for removed in old_set.difference(&new_set) {
                let user = user_map.get(removed).unwrap();
                if user.realm != "csh" {
                    continue;
                }
                pings
                    .send_remove(
                        &user.email.trim_end_matches("@csh.rit.edu"),
                        &driver.name,
                        &event_name,
                    )
                    .await;
            }
            for added in new_set.difference(&old_set) {
                let user = user_map.get(added).unwrap();
                if user.realm != "csh" {
                    continue;
                }
                pings
                    .send_add(
                        &user.email.trim_end_matches("@csh.rit.edu"),
                        &driver.name,
                        &event_name,
                    )
                    .await;
            }
        }
    }
    Ok(())
}

pub async fn work_loop(
    mut db: MultiplexedConnection,
    work_queue: WorkQueue,
    db_pool: Pool<Postgres>,
    pings: PingClient,
) -> RedisResult<()> {
    loop {
        // Wait for a job with no timeout and a lease time of 5 seconds.
        let job: Item = work_queue
            .lease(&mut db, None, Duration::from_secs(5))
            .await?
            .unwrap();
        match work(&job, &db_pool, &pings).await {
            // Mark successful jobs as complete
            Ok(()) => {
                work_queue.complete(&mut db, &job).await?;
            }
            // Drop a job that should be retried - it will be returned to the work queue after
            // the (5 second) lease expires.
            Err(err) if err.should_retry => (),
            // Errors that shouldn't cause a retry should mark the job as complete so it isn't
            // tried again.
            Err(err) => {
                work_queue.complete(&mut db, &job).await?;
            }
        }
    }
}
