use lazy_static::lazy_static;
pub extern crate redis;
use std::time::Duration;
use redis::{Client,Connection};
use super::config::CONF;

lazy_static! {
    // lazy_static的公共变量必须是大写
    pub static ref REDIS :Client = init_redis();
    // pub static ref MONGO :
}

fn init_redis() ->Client {
    let db :i64;
    match CONF.redis.db{
        Some(val) => {db = val as i64},
        None => {db = 0},
    }
    // connect to redis
    let option = redis::ConnectionInfo{
        addr:Box::new(redis::ConnectionAddr::Tcp(CONF.redis.addr.as_ref().unwrap().to_string(),CONF.redis.port.unwrap() as u16)),
        db:db,
        username:handle_str_2_none(CONF.redis.user.clone()),
        passwd:handle_str_2_none(CONF.redis.passwd.clone()),
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



fn handle_str_2_none(args :Option<String>)->Option<String>{
    match args{
        Some(val) => {
            if val==""{
                None
            }else{
                Some(val)
            }},
            None =>{
                None
            },
    }
}