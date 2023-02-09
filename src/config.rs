use std::{path::Path, fs::File, io::Read};
use serde_json::from_str;
use crate::models;

impl models::Config {
    pub fn get_uid(&mut self) {
        get_uid(&mut self.url);
    }
}

pub fn config()-> std::io::Result<models::Config>{
    let path_config = Path::new("./config/config.json");
    let mut contents = String::new();
    let mut config_file = match File::open(path_config){
        Ok(file) => file,
        Err(error)=>{
            return Err(error);
        }
    };
    config_file.read_to_string(&mut contents)?;
    let config:models::Config = from_str(&contents)?;
    if config.webhook.is_empty(){
        println!("Please add url of your webhook");
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Please add url of your webhook",
        ));
    }
    Ok(config)
}

fn get_uid(url: &mut [String]){
    for u in url.iter_mut(){
        let split:Vec<&str> = u.split('=').collect();
        let last = split.last().unwrap().to_owned();
        *u = last.to_string();
    }
}
