mod formats;
mod http_auth;
mod influxdb;

use std::env;
use std::error::Error;
use chrono::prelude::*;
use std::ops::Add;
use chrono::Duration;
use std::collections::HashMap;
use log::{info, debug};
use env_logger::Env;

use formats::MyenergiZappiData;

fn main() -> Result<(), Box<dyn Error>> {

    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    let uri = "https://s2.myenergi.net";

    let zappi_sn = env::var("ZAPPI_SN")?;

    let username = env::var("MYENERGI_USERNAME")?;

    let password = env::var("MYENERGI_PASSWORD")?;

    let influxdb_username = env::var("INFLUXDB_USERNAME")?;

    let influxdb_password = env::var("INFLUXDB_PASSWORD")?;

    let influxdb_url = env::var("INFLUXDB_URL")?;

    let args: Vec<String> = env::args().collect();

    debug!("Arguments: {:?}", args);

    let complete_datetime = args.get(1).map(|d| format!("{} 00:00:00", d));

    let report_date = complete_datetime
        .map(|d| Utc.datetime_from_str(&d, "%Y-%m-%d %H:%M:%S"))
        .unwrap_or(Ok(Utc::now().add(Duration::days(-1))))?;
    info!("Processing report for day {}", report_date);

    let myenergi_date = formats::myenergi_date(&report_date);

    let url = format!("{}/cgi-jday-Z{}-{}", uri, zappi_sn, myenergi_date);

    let client = reqwest::blocking::Client::new();
    let auth_header =
        http_auth::get_auth_header_for(&client, uri, &username, &password)?;

    let response = client.get(&url).header(reqwest::header::AUTHORIZATION, &auth_header).send()?;

    println!("{}", &response.status().as_str());

    let raw_daily_report: HashMap<String, Vec<MyenergiZappiData>> = response.json()?;

    let daily_report = formats::fix_daily_report(raw_daily_report);

    // let daily_report: HashMap<String, Vec<FixedMyenergiData>> = raw_daily_report.into();
    println!("{:?}", daily_report);

    for (device, report) in daily_report.iter() {
        influxdb::publish_zappi_report(&client, &influxdb_url, &influxdb_username, &influxdb_password,
        device, report)?;
    }

    Ok(())
}
