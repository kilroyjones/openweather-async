use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
//Current hac with double since #[serde(rename="lowercase")] doesn't seem to work
pub struct Coord {
    pub Lon: Option<f32>,
    pub Lat: Option<f32>,
    pub lon: Option<f32>,
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
    pub sunrise: u32,
    pub sunset: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub coord: Coord,
    pub weather: Option<Vec<WeatherData>>,
    pub base: Option<String>,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub rain: Option<String>,
    pub snow: Option<String>,
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
    cod: u32,
    calctime: f32,
    cnt: u32,
    list: Vec<Weather>,
}


