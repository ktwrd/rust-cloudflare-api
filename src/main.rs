mod cloudflare_api
{
    pub use cloudflare_api::auth;
}

fn main() {    
    let mut authObj = cloudflare_api::auth::TokenAuth{email: String::from("dariox.club@gmail.com"), token: String::from("0123456789")};
    println!("Hello, world!");
}
