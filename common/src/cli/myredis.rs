use super::REDIS;
use std::time::Duration;
use redis::Connection;
use crate::config::CONF;

pub fn get_conn()->Connection{
    let conn = REDIS.get_connection().unwrap();

    match CONF.redis.read_timeout{
        Some(val) => {
            if val!=0{
                let _ = conn.set_read_timeout(Some(Duration::from_secs(val as u64)));
            }
        },
        None => {}
    }

    match CONF.redis.write_timeout{
        Some(val) => {
            if val!=0{
                let _ = conn.set_write_timeout(Some(Duration::from_secs(val as u64)));
            }
        },
        None => {}
    }

    conn
}