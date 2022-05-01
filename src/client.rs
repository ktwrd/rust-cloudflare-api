use crate::auth as auth;
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseResultInfo
{
    page: i32,
    per_page: i32,
    total_pages: i32,
    count: i32,
    total_count: i32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZoneInfoOwner
{
    id: String,
    email: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZoneInfoAccount
{
    id: String,
    name: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZoneInfoMeta
{
    step: i32,
    custom_certificate_quota: i32,
    page_rule_quota: i32,
    phishing_detected: bool,
    multiple_railguns_allowed: bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZonePlan
{
    id: String,
    name: String,
    price: Option<i32>,
    currency: Option<String>,
    frequency: Option<String>,
    is_subscribed: Option<bool>,
    can_subscribe: Option<bool>,
    legacy_id: Option<String>,
    legacy_discount: Option<bool>,
    externally_managed: Option<bool>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZoneInfo
{
    id: String,
    name: String,
    status: String,
    paused: bool,
    development_mode: i32,
    name_servers: Vec<String>,
    original_name_servers: Option<Vec<String>>,
    original_registrar: Option<String>,
    original_dnshost: Option<String>,
    modified_on: String,
    activated_on: String,
    meta: ResponseZoneInfoMeta,
    owner: ResponseZoneInfoOwner,
    account: ResponseZoneInfoAccount,
    permissions: Vec<String>,
    plan: Option<ResponseZonePlan>
}

pub struct Client
{
    auth: auth::TokenAuth,
    web_client: reqwest::Client,
    pub base_url: String
}
impl Client
{
    pub fn new(auth: auth::TokenAuth) -> Client
    {
        Client
        {
            auth: auth,
            web_client: reqwest::Client::new(),
            base_url: String::from("https://api.cloudflare.com/client/v4")
        }
    }

    pub async fn verify_token(&mut self) -> Result<String, task::JoinError>
    {
        let resp: std::result::Result<reqwest::Response, reqwest::Error> =
            self.web_client.get([
                    self.base_url.to_string(),
                    String::from("user/tokens/verify")
                    ].join("/"))
            .headers(self.auth.headers())
            .send()
            .await;
            
        let thing: reqwest::Result<String> = resp.unwrap().text().await;
        let result = thing.unwrap();
        Ok(result)
    }

    #[allow(dead_code)]
    pub async fn get_zone(&mut self, zone: String) -> Result<String, task::JoinError>
    {
        let resp: std::result::Result<reqwest::Response, reqwest::Error> =
            self.web_client.get([
                    self.base_url.to_string(),
                    String::from("zones"),
                    zone].join("/"))
            .headers(self.auth.headers())
            .send()
            .await;
            
        let thing: reqwest::Result<String> = resp.unwrap().text().await;
        let result = thing.unwrap();
        Ok(result)
    }

    pub async fn get_all_zones(&mut self) -> Result<Vec<ResponseZoneInfo>, task::JoinError>
    {
        let resp: std::result::Result<reqwest::Response, reqwest::Error> = 
            self.web_client.get([
                self.base_url.to_string(),
                String::from("zones")
            ].join("/"))
            .headers(self.auth.headers())
            .send()
            .await;
        
        let thing: reqwest::Result<String> = resp.unwrap().text().await;
        let result = thing.unwrap();
        println!("{:?}", result);

        let jsonValue: serde_json::Value = serde_json::from_str(&result).unwrap();

        let mut finalResult: Vec<ResponseZoneInfo> = vec![];

        if jsonValue["success"] == true
        {   
            let mut result_data: Vec<ResponseZoneInfo> = vec![];
            if jsonValue["result"].is_array() == true
            {
                result_data = serde_json::from_str(&jsonValue["result"].to_string()).unwrap();
            }
            // let result_info: ResponseResultInfo = serde_json::from_str(&jsonValue["result_info"].to_string()).unwrap();

            // println!("result_info: {:?}", result_info);
            // let mut index: i128 = 0;
            // for value in result_data.iter() {
            //     println!("{}: {:#?}", index, value);
            //     index += 1;
            // }

            finalResult = result_data;
        }

        Ok(finalResult)
    }
}