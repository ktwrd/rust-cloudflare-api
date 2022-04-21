use std::collections::HashMap;

pub struct TokenAuth
{
    email: String,
    token: String
}
impl TokenAuth
{
    fn get_headers() -> HashMap<String, String>
    {
        let mut headers = HashMap::new();
        headers.insert(String::from("X-Auth-Email"), email);
        headers.insert(String::from("Authorization"), String::from("Bearer ") + token);
        Ok(headers)
    }
}