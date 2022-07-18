use reqwest::header::{HeaderMap, HeaderName, AUTHORIZATION};

pub struct TokenAuth
{
    pub email: String,
    pub token: String
}
impl TokenAuth
{
    pub fn headers(&self) -> HeaderMap
    {
        let mut headers = HeaderMap::new();
        
        if self.token.len() < 1 {
            panic!("Token is not set");
        }
        if self.email.len() < 1 {
            panic!("Email is not set");
        }

        headers.insert(AUTHORIZATION, [&"Bearer", self.token.as_str()].join(" ").parse().unwrap());
        headers.insert(HeaderName::from_static("x-auth-email"), self.email.parse().unwrap());
        headers
    }
}
