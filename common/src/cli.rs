use lazy_static::lazy_static;
pub extern crate redis;
use std::time::Duration;
use redis::{Client,Connection};

lazy_static! {
    // lazy_static的公共变量必须是大写
    pub static ref REDIS :Client = init_redis();
    // pub static ref MONGO :
}

fn init_redis() ->Client {
    // connect to redis
    let option = redis::ConnectionInfo{
        addr:Box::new(redis::ConnectionAddr::Tcp("10.0.0.142".to_string(),6379)),
        db:1,
        username:None,
        passwd:None,
    };
    
    let client = redis::Client::open(option);
    // debug!("connect redis success");
    client.unwrap()
}

pub fn get_conn()->Connection{
    let conn = REDIS.get_connection().unwrap();
    let _ = conn.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = conn.set_write_timeout(Some(Duration::from_secs(5)));

    conn
}

