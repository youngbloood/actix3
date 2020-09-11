// #[cfg(feature = "sync")]
// use mongodb::Client;
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



fn handle_str_2_none(args :Option<String>,default :Option<String>)->Option<String>{
    match args{
        Some(val) => {
            if val==""{
                match default{
                    None =>None,
                    Some(val) => Some(val),
                }
            }else{
                Some(val)
            }},
        None =>{
            None
        },
    }
}

// // 去掉空值
// pub fn rm_none<T :i8+i16+i32+i64+i128+u8+u16+u32+u64+u128+String>(args :Option<T>,default :Option<T>)->Option<T>{
//     match args{
//         Some(val) => {
//             if val==""{
//                 match default{
//                     None =>None,
//                     Some(val) => Some(val),
//                 }
//             }else{
//                 Some(val)
//             }},
//         None =>{
//             None
//         },
//     }
// }