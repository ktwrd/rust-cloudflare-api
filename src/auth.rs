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
        
        headers.insert(AUTHORIZATION, [&"Bearer", self.token.as_str()].join(" ").parse().unwrap());
        headers.insert(HeaderName::from_static("x-auth-email"), self.email.parse().unwrap());
        headers
    }

}