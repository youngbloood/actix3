#[warn(unused_imports)]
pub use mongodb;
#[cfg(feature = "sync")]
use mongodb::Client;

use mongodb::options::{StreamAddress,ClientOptions};
use crate::config::CONF;
use std::string::String;
use crate::cli::handle_str_2_none;





#[cfg(feature = "sync")]
pub fn init_mongo() -> Client {

    let options = ClientOptions::builder()
                      .hosts(vec![
                           StreamAddress {
                            hostname: handle_str_2_none(CONF.mongo.addr,Some("localhost")).unwrap(),
                            port:CONF.mongo.port,
                           }
                       ])
                       .connect_timeout(CONF.mongo.connect_timeout)
                       .build();
    
    let client = Client::with_options(options)?;
    Ok(client)
}