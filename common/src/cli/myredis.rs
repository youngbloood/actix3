use super::REDIS;
use std::time::Duration;
use redis::{Client,Connection};
use crate::config::CONF;
use crate::cli;


// 初始化redis
pub fn init_redis() ->Client {
    let db :i64;
    match CONF.redis.db{
        Some(val) => {db = val as i64},
        None => {db = 0},
    }
    // connect to redis
    let option = redis::ConnectionInfo{
        addr:Box::new(redis::ConnectionAddr::Tcp(CONF.redis.addr.as_ref().unwrap().to_string(),CONF.redis.port.unwrap() as u16)),
        db:db,
        username:cli::handle_str_2_none(CONF.redis.user.clone()),
        passwd:cli::handle_str_2_none(CONF.redis.passwd.clone()),
    };

    
    let client = redis::Client::open(option);
    // debug!("connect redis success");
    client.unwrap()
}

// 获取一个redis connetion
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