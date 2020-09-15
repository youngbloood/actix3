pub extern crate redis;
use lazy_static::lazy_static;
pub mod myredis;
pub mod mymongo;

use redis::Client as redisClient;



lazy_static! {
    // lazy_static的公共变量必须是大写
    pub static ref REDIS :redisClient = myredis::init_redis();
   
}

#[cfg(feature = "sync")]
lazy_static!{
    pub static ref MONGO :mymongo::Client = mymongo::init_mongo();
}


// 去掉空值
pub fn rm_none<T:Default+Eq>(args :Option<T>,default_arg :Option<T>)->Option<T> {
    match args{
        Some(val) => {
            if T::default().eq(&val){
                default_arg
            }else{
                Some(val)
            }
        },
        None => {
            None
        },
    }
}