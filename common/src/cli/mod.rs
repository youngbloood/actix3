pub extern crate redis;
use lazy_static::lazy_static;
pub mod myredis;
pub mod mymongo;
use redis::Client;


lazy_static! {
    // lazy_static的公共变量必须是大写
    pub static ref REDIS :Client = myredis::init_redis();
    // pub static ref MONGO :
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