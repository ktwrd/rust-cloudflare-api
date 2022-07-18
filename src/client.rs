use crate::auth as auth;
use tokio::task;

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
}