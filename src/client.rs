use crate::auth as auth;
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseResultInfo
{
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub count: i32,
    pub total_count: i32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZoneInfoOwner
{
    pub id: String,
    pub email: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZoneInfoAccount
{
    pub id: String,
    pub name: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZoneInfoMeta
{
    pub step: i32,
    pub custom_certificate_quota: i32,
    pub page_rule_quota: i32,
    pub phishing_detected: bool,
    pub multiple_railguns_allowed: bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZonePlan
{
    pub id: String,
    pub name: String,
    pub price: Option<i32>,
    pub currency: Option<String>,
    pub frequency: Option<String>,
    pub is_subscribed: Option<bool>,
    pub can_subscribe: Option<bool>,
    pub legacy_id: Option<String>,
    pub legacy_discount: Option<bool>,
    pub externally_managed: Option<bool>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseZoneInfo
{
    pub id: String,
    pub name: String,
    pub status: String,
    pub paused: bool,
    pub development_mode: i32,
    pub name_servers: Vec<String>,
    pub original_name_servers: Option<Vec<String>>,
    pub original_registrar: Option<String>,
    pub original_dnshost: Option<String>,
    pub modified_on: String,
    pub activated_on: String,
    pub meta: ResponseZoneInfoMeta,
    pub owner: ResponseZoneInfoOwner,
    pub account: ResponseZoneInfoAccount,
    pub permissions: Vec<String>,
    pub plan: Option<ResponseZonePlan>
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

    pub async fn get_zone_arr(&mut self) -> Result<String, task::JoinError>
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
        Ok(result)
    }

    #[allow(dead_code)]
    pub async fn get_zone(&mut self, zone: String) -> Result<Option<ResponseZoneInfo>, task::JoinError>
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

        let mut final_response: Option<ResponseZoneInfo> = None;

        let response_data: serde_json::Value = serde_json::from_str(&result).unwrap();
        if response_data["success"] == true
        {
            let result_data: ResponseZoneInfo = serde_json::from_str(&response_data["result"].to_string()).unwrap();
            final_response = Option::Some(result_data);
        }

        Ok(final_response)
    }

    pub async fn get_all_zone_records(&mut self, zone: String) -> Result<String, task::JoinError>
    {
        let resp: std::result::Result<reqwest::Response, reqwest::Error> = 
            self.web_client.get([
                self.base_url.to_string(),
                String::from("zones"),
                zone,
                String::from("dns_records")
            ].join("/"))
            .headers(self.auth.headers())
            .send()
            .await;
        let thing: reqwest::Result<String> = resp.unwrap().text().await;
        let result: String = thing.unwrap();

        let response_data: serde_json::Value = serde_json::from_str(&result).unwrap();
        if response_data["success"] == true
        {
            let result_data: serde_json::Value = serde_json::from_str(&response_data["result"].to_string()).unwrap();
        }

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