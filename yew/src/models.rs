use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct DeviceDetails {
    #[serde(rename = "wifisignal")]
    pub wifi_signal: Value,
    pub version: Value,
    pub addr: Vec<Value>,
    #[serde(rename = "lastseen")]
    pub last_seen: Value,
    #[serde(rename = "boilerOn")]
    pub boiler_on: Value,
    #[serde(rename = "dhwMode")]
    pub dhw_mode: Value,
    #[serde(rename = "tFLO")]
    pub t_flo: Value,
    #[serde(rename = "tdH")]
    pub t_dh: Value,
    #[serde(rename = "tESt")]
    pub t_est: Value,
}

fn deserialize_days<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, Vec<u8>> = HashMap::deserialize(deserializer)?;
    let mut days = vec![vec![]; 7];
    for (key, value) in map {
        if let Ok(index) = key.parse::<usize>() {
            if index < 7 {
                days[index] = value;
            }
        }
    }
    Ok(days)
}

fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        Value::Bool(b) => Ok(b),
        Value::Number(n) => Ok(n.as_i64().unwrap_or(0) != 0),
        _ => Ok(false),
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Room {
    pub temp: i32,
    pub t1: i32,
    pub t2: i32,
    pub t3: i32,
    pub settemp: i32,
    pub units: i32,
    pub lowbattery: i32,
    pub cmdissued: i32,
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub heating: bool,
    pub winter: i32,
    pub advance: i32,
    pub boost: i32,
    pub fakeboost: i32,
    pub mode: i32,
    #[serde(deserialize_with = "deserialize_days")]
    pub days: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RoomHistoryPoint {
    pub ts: String,
    pub temp: f64,
    pub settemp: f64,
    pub heating: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct WeatherHistoryPoint {
    pub ts: String,
    pub temp: f64,
}
