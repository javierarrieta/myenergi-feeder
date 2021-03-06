use std::error::Error;
use reqwest::blocking::Client;
use crate::formats::FixedMyenergiData;


pub fn publish_zappi_report(client: &Client, url: &str, influxdb_username: &str, influxdb_password: &str,
                            device: &str, data: &Vec<FixedMyenergiData>) -> Result<(), Box<dyn Error>> {
    for datum in data.iter() {
        publish_zappi_data(client, url, influxdb_username, influxdb_password, device, datum)?;
    }

    return Ok(());
}

fn publish_zappi_data(client: &Client, url: &str, influxdb_username: &str, influxdb_password: &str,
                      device: &str, data: &FixedMyenergiData) -> Result<(), Box<dyn Error>> {

                        let timestamp: i64 = data.datetime.timestamp_nanos();

    client.post(url).basic_auth(influxdb_username, Some(influxdb_password))
        .body(format!("power,device={},circuit=grid value={:.3} {}", device, data.imported, timestamp)).send()?;

    client.post(url).basic_auth(influxdb_username, Some(influxdb_password))
        .body(format!("power,device={},circuit=zappi value={:.3} {}", device, data.zappi1, timestamp)).send()?;

    client.post(url).basic_auth(influxdb_username, Some(influxdb_password))
        .body(format!("voltage,device={},circuit=grid value={:.3} {}", device, data.voltage, timestamp)).send()?;

    client.post(url).basic_auth(influxdb_username, Some(influxdb_password))
        .body(format!("frequency,device={},circuit=grid value={:.3} {}", device, data.frequency, timestamp)).send()?;

    return Ok(());
}