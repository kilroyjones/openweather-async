pub use crate::units::*;
pub use crate::models::*;
use reqwest::{Client, Result};
use serde::de::DeserializeOwned;

pub struct OpenWeather {
    pub api_key: String,
    pub units: Units,
    client: Client,
}


impl OpenWeather {
    pub async fn new(
        api_key: &str,
        units: Units
    ) -> Result<OpenWeather> {
        Ok(OpenWeather {
            api_key: api_key.to_string(),
            units: units,
            client: Client::new(),
        })
    }

    pub async fn get_by_city(
        &self,
        city: &str
    ) -> Result<Weather> {
        self.query(&format!("weather?q={}", &city)).await
    }


    pub async fn get_by_city_and_country(
        &self,
        city: &str,
        country: &str
    ) -> Result<Weather> {
        self.query(&format!("weather?q={},{}", &city, &country)).await
    }

    pub async fn get_by_coordinates(
        &self,
        lat: f32,
        lon: f32
    ) -> Result<Weather> {
        self.query(&format!("weather?lat={}&lon={}", lat, lon)).await
    }

    pub async fn get_by_zipcode(
        &self,
        zipcode: u32,
        country: &str,
    ) -> Result<Weather> {
        self.query(&format!("weather?q={},{}", zipcode, country)).await
    }

    pub async fn get_by_cities_in_zone(
        &self,
        lon1: f32,
        lat1: f32,
        lon2: f32,
        lat2: f32,
        zoom: u32,
    ) -> Result<WeatherMultiple> {
        self.query(&format!("box/city?bbox={},{},{},{},{}", lon1, lat1, lon2, lat2, zoom)).await
    }

    pub async fn get_by_cities_in_cycle(
        &self,
        lon: f32,
        lat: f32,
        count: u32,
    ) -> Result<WeatherMultiple> {
        self.query(&format!("find?lat={}&lon={}&cnt={}", lon, lat, count)).await
    }


    fn format_addr(&self, query: &str) -> String {
        let base_http = "https://api.openweathermap.org/data/2.5/";
        format!(
            "{}{}&appid={}&units={}",
            &base_http, &query, self.api_key, self.units.value()
        )
    }

    async fn query<T: DeserializeOwned>(&self, query: &str) -> Result<T> {
        let addr = self.format_addr(query);
        let res2 = self.client
            .get(addr.clone())
            .send()
            .await?;
        let txt = res2.text().await?;
        println!("T: {}", txt);



        let res = self.client
            .get(addr)
            .send()
            .await?;

        let data = res.json().await?;
        Ok(data)
    }
}
