use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub fn myenergi_date(utc: &DateTime<Utc>) -> String {
    return format!("{}-{}-{}", utc.year(), utc.month(), utc.day());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyenergiZappiData {
    hr: Option<u8>,
    min: Option<u8>,
    dom: u8,
    mon: u8,
    yr: u16,
    imp: i32,
    v1: u32,
    frq: u32,
    nect1: Option<i32>,
    h1b: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FixedMyenergiData {
    pub datetime: DateTime<Utc>,
    pub imported: f32,
    pub voltage: f32,
    pub frequency: f32,
    pub clamp1: f32,
    pub zappi1: f32,
}

impl From<&MyenergiZappiData> for FixedMyenergiData {
    fn from(data: &MyenergiZappiData) -> Self {
        FixedMyenergiData {
            datetime: Utc.ymd(data.yr as i32, data.mon as u32, data.dom as u32)
                .and_hms(data.hr.unwrap_or(0) as u32, data.min.unwrap_or(0) as u32, 0),
            imported: data.imp as f32 / 60.0,
            voltage: data.v1 as f32 / 10.0,
            frequency: data.frq as f32 / 100.0,
            clamp1: data.nect1.map(|n| n as f32 / 60.0).unwrap_or(0.0),
            zappi1: data.h1b.map(|n| n as f32 / 60.0).unwrap_or(0.0),
        }
    }
}

pub fn fix_daily_report(daily_report: HashMap<String, Vec<MyenergiZappiData>>) -> HashMap<String, Vec<FixedMyenergiData>> {
    return daily_report.iter().map(|(key, value)| {
        let new_value: Vec<FixedMyenergiData> = value.iter().map(|r| r.into()).collect();
        (key.to_owned(), new_value)
    }).collect();
}