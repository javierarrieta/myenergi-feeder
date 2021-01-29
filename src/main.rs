use std::env;
use std::error::Error;
use digest_auth::{AuthorizationHeader, AuthContext};

fn main() -> Result<(), Box<dyn Error>> {

    let uri = "https://s2.myenergi.net";

    let zappi_sn = env::var("ZAPPI_SN")?;

    let username = env::var("MYENERGI_USERNAME")?;

    let password = env::var("MYENERGI_PASSWORD")?;

    println!("Here!");

    let myenergi_date = "2021-01-28";

    let url = format!("{}/cgi-jday-Z{}-{}", uri, zappi_sn, myenergi_date);

    let client = reqwest::blocking::Client::new();

    let response_for_auth = client.get(&url).send()?;

    let auth_prompt = response_for_auth.headers().get("www-authenticate")
        .ok_or("No www-authenticate header present")?
        .to_str()?;

    let mut prompt = digest_auth::parse(auth_prompt)?;

    let ctx = AuthContext::new(username, password, &url);

    let auth_header = prompt.respond(&ctx)?.to_header_string();

    let response = client.get(&url).header(reqwest::header::AUTHORIZATION, &auth_header).send()?;

    println!("{}", response.status().as_str());

    println!("{}", response.text()?);

    Ok(())
}
