use chrono::prelude::*;


pub fn myenergi_date(utc: &DateTime<Utc>) -> String {
    return format!("{}-{}-{}", utc.year(), utc.month(), utc.day());
}