use reqwest::blocking::Client;
use digest_auth::AuthContext;

pub fn get_auth_header_for(client: &Client, url: &str, username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response_for_auth = client.get(url).send()?;

    let auth_prompt = response_for_auth.headers().get("www-authenticate")
        .ok_or("No www-authenticate header present")?
        .to_str()?;

    let mut prompt = digest_auth::parse(auth_prompt)?;

    let ctx = AuthContext::new(username, password, url);

    return Ok(prompt.respond(&ctx)?.to_header_string());
}