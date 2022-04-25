mod auth;
mod client;
mod configparse;
use tokio::task;

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

    Ok(())
}
