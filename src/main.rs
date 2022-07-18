mod auth;
mod client;
mod configparse;
use tokio::task;
use std::{collections::{HashMap}, array};
use serde_json::{Value};

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Deserialize, Debug)]
struct Item {
    value: u64,
}

#[tokio::main]
async fn main() -> Result<(), task::JoinError>
{
    let mut config = configparse::INIFile::new(String::from("./config.cfg"));
    // Initalize the Configuration
    config.data.insert(String::from("cloudflare_email"), String::from(""));
    config.data.insert(String::from("cloudflare_token"), String::from(""));
    config.read().unwrap();

    let config_email_obj = config.data.get(&String::from("cloudflare_email"));
    let mut config_email = String::from("");
    if config_email_obj.is_none() == false {
        config_email = config_email_obj.unwrap().to_string()
    }
    let config_token_obj = config.data.get(&String::from("cloudflare_token"));
    let mut config_token = String::from("");
    if config_token_obj.is_none() == false {
        config_token = config_token_obj.unwrap().to_string()
    }

    let auth = auth::TokenAuth
    {
        email: config_email,
        token: config_token
    };

    let mut client = client::Client::new(auth);
    println!("baseURL: {}", client.base_url);

    let verify_response: String = client.verify_token().await.unwrap();
    println!("verifyResponse: {}", verify_response);

    let getZoneArrRes: String = client.get_zone_arr().await.unwrap();
    let getZoneArrRes_str: &str = &getZoneArrRes[..];
    let getZoneArrRes_map: HashMap<String, Value> = serde_json::from_str(getZoneArrRes_str).unwrap();
    println!("getZoneArrRes: {:#?}", getZoneArrRes_map);
    println!("success: {}", getZoneArrRes_map.get(&String::from("success")).unwrap());
    let responseArr = getZoneArrRes_map.get(&String::from("result")).unwrap().as_array().unwrap();
    
    for (i, elem) in responseArr.iter().enumerate() {
        let name = elem.get(&String::from("name")).unwrap().to_string();
        let id = elem.get(&String::from("id")).unwrap().to_string();
        print!("{}: ", id);
        print!("{}\n", name);
    }

    Ok(())
}
