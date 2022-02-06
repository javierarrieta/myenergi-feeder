use reqwest::IntoUrl;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use digest_auth::AuthContext;

pub fn get_auth_header_for(path: &str, response: &Response, username: &str, password: &str) 
                                                    -> Result<String, Box<dyn std::error::Error>> {
    
    let auth_prompt = response.headers().get("www-authenticate")
        .ok_or("No www-authenticate header present")?
        .to_str()?;
    let mut prompt = digest_auth::parse(auth_prompt)?;

    let ctx = AuthContext::new(username, password, path);

    return Ok(prompt.respond(&ctx)?.to_header_string());
}

pub fn get_with_digest_auth<U: IntoUrl + Copy>(client: &Client, u: U, username: &str, password: &str) 
                                                    -> Result<Response, Box<dyn std::error::Error>> {
    let url = u.into_url()?;
    let response = client.get(u).send()?;
    if response.status().as_str() == "401" && response.headers().get("www-authenticate").is_some() {
        let auth_header = get_auth_header_for(&url.path(), &response, username, password)?;
        return client.get(url).header(reqwest::header::AUTHORIZATION, auth_header).send().map_err(|e| e.into())
    } else {
        return Ok(response)
    }
}