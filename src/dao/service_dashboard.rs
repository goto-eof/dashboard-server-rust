use std::net::SocketAddr;

use chrono::{NaiveDateTime, Utc};
use log::debug;

use crate::DB_POOL;
pub async fn update_ip(opt_address: Option<SocketAddr>) -> bool {
    let pool = DB_POOL.get().await;
    if opt_address.is_none() {
        return false;
    }

    let result = sqlx::query!("select key from dshbrd_info where key = 'ip'")
        .fetch_one(pool)
        .await;

    debug!("result: {:?}", result);
    if result.is_ok() {
        let ts = Utc::now().naive_utc();
        let result = sqlx::query!(
            "UPDATE dshbrd_info SET value = $1, updated_at=$2 where key='ip'",
            opt_address.unwrap().ip().to_string(),
            ts
        )
        .execute(pool)
        .await;
        if result.is_err() {
            debug!("Error updating ip! {:?}", result.err());
        }
        return true;
    }

    let result = sqlx::query!(
        "insert into dshbrd_info (key, value) values('ip', $1)",
        opt_address.unwrap().ip().to_string(),
    )
    .bind(opt_address.unwrap().ip().to_string())
    .execute(pool)
    .await;
    if result.is_err() {
        debug!("Error inserting ip!: {:?}", result.err());
    }
    return true;
}

#[derive(serde::Serialize, Default)]
pub struct KeyValue {
    pub key: Option<String>,
    pub value: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub async fn retrieve_ip_from_db() -> KeyValue {
    let pool = DB_POOL.get().await;
    let result =
        sqlx::query!("select key, value, created_at, updated_at from dshbrd_info where key = 'ip'")
            .fetch_one(pool)
            .await;
    if result.is_err() {
        return KeyValue::default();
    }
    let result = result.unwrap();
    return KeyValue {
        key: result.key,
        value: result.value,
        created_at: result.created_at,
        updated_at: result.updated_at,
    };
}
