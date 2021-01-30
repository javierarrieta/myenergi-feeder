mod formats;
mod http_auth;

use std::env;
use std::error::Error;
use chrono::prelude::*;
use std::ops::Add;
use chrono::Duration;
use std::collections::HashMap;

use formats::MyenergiZappiData;

fn main() -> Result<(), Box<dyn Error>> {

    let uri = "https://s2.myenergi.net";

    let zappi_sn = env::var("ZAPPI_SN")?;

    let username = env::var("MYENERGI_USERNAME")?;

    let password = env::var("MYENERGI_PASSWORD")?;

    let myenergi_date = formats::myenergi_date(&Utc::now().add(Duration::days(-1)));

    let url = format!("{}/cgi-jday-Z{}-{}", uri, zappi_sn, myenergi_date);

    let client = reqwest::blocking::Client::new();
    let auth_header =
        http_auth::get_auth_header_for(&client, uri, &username, &password)?;

    let response = client.get(&url).header(reqwest::header::AUTHORIZATION, &auth_header).send()?;

    println!("{}", &response.status().as_str());

    // println!("{:?}", &response.text()?);

    let raw_daily_report: HashMap<String, Vec<MyenergiZappiData>> = response.json()?;

    let daily_report = formats::fix_daily_report(raw_daily_report);

    // let daily_report: HashMap<String, Vec<FixedMyenergiData>> = raw_daily_report.into();
    println!("{:?}", daily_report);

    Ok(())
}
