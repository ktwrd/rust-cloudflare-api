mod auth;
mod client;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), task::JoinError>
{
    let auth = auth::TokenAuth
    {
        email: String::from("dariox.club@gmail.com"),
        token: String::from("")
    };

    let mut client = client::Client::new(auth);
    println!("baseURL: {}", client.base_url);

    let verify_response: String = client.verify_token().await.unwrap();
    println!("verifyResponse: {}", verify_response);
    Ok(())
}
