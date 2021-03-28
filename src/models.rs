use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{self};
use std::fmt;


#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    #[serde(rename = "Lon")]
    pub lon: Option<f32>,
    #[serde(rename = "Lat")]
    pub lat: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(deserialize = "Weather", serialize = "Weather"))]
pub struct WeatherData {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub sea_level: Option<f32>,
    pub grnd_level: Option<f32>,
    pub temp_kf: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
    pub gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub _1h: Option<f32>,
    #[serde(rename = "3h")]
    pub _3h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snow {
    #[serde(rename = "1h")]
    pub _1h: Option<f32>,
    #[serde(rename = "3h")]
    pub _3h: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: Option<u32>,
    pub today: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub message_type: Option<u32>,
    pub id: Option<u32>,
    pub country: String,
    pub sunrise: Option<u32>,
    pub sunset: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub coord: Coord,
    pub weather: Option<Vec<WeatherData>>,
    pub base: Option<String>,
    pub main: Main,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub clouds: Clouds,
    pub dt: u32,
    pub sys: Option<Sys>,
    pub timezone:Option<i32>,
    pub id: i32,
    pub name: String,
    pub cod: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherMultiple {
    #[serde(deserialize_with = "derserialize_u32_or_string")]
    cod: u32,
    calctime: Option<f32>,
    count: Option<f32>,
    cnt: Option<u32>,
    list: Vec<Weather>,
}


//https://stackoverflow.com/questions/37870428/convert-two-types-into-a-single-type-with-serde
struct Deserializeu32orString;

impl<'de> de::Visitor<'de> for Deserializeu32orString {
    type Value = u32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("sdfaan integer or a string")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>

    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(0)
    }
}

fn derserialize_u32_or_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(Deserializeu32orString)
}


