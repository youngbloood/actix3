extern crate toml;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref CONF :All= init_config("./config.toml");
}

fn init_config(file_path:&str)->All{

    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s
        ,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    let config: All = toml::from_str(&str_val).unwrap();

    config
}


#[derive(Deserialize)]
#[derive(Debug,Clone)]
pub struct All{
    pub redis :Redis,
    pub http_port:Option<i32>
}



#[derive(Debug,Clone)]
#[derive(Deserialize)]
pub struct Redis{
    pub addr :Option<String>,
    pub port :Option<i32>,
    pub db :Option<i32>,
    pub user :Option<String>,
    pub passwd:Option<String>
}
