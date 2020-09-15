#[warn(unused_imports)]
pub use mongodb;
#[cfg(feature = "sync")]
use mongodb::{Client,Collection};

use mongodb::options::{StreamAddress,ClientOptions};
use crate::config::CONF;
use crate::cli;

#[cfg(feature = "sync")]
use crate::cli::MONGO;



#[cfg(feature = "sync")]
pub fn init_mongo() -> Client {

    let sa = StreamAddress {
        hostname: cli::rm_none(CONF.mongo.addr,Some("localhost")).unwrap(),
        port:cli::rm_none(CONF.mongo.port,Some(27017)).unwrap(),
       };
    
       println!("mongo配置  {:?}",sa);

    let options = ClientOptions::builder()
                      .hosts(vec![
                           StreamAddress {
                            hostname:CONF.mongo.addr.unwrap(),
                            port:cli::rm_none(CONF.mongo.port,Some(27017)),
                           }
                       ])
                       .connect_timeout(CONF.mongo.connect_timeout)
                       .build();
    
    let client = Client::with_options(options)?;
    Ok(client)
}


#[cfg(feature = "sync")]
pub fn get_conn(db :&str,collection:&str)->Collection{
    let coll = MONGO.database(db).collection(collection);
    coll
}