pub use crate::units::*;
pub use crate::models::*;
use std::error::Error;


pub struct OpenWeather {
    pub api_key: String,
    pub units: Units,
}


impl OpenWeather {

    pub async fn new(
        api_key: &str,
        units: Units
    ) -> Result<OpenWeather, Box<dyn std::error::Error>> {
        Ok(OpenWeather {
            api_key: api_key.to_string(),
            units: units,
        })
    }

    async fn get_multiple(
        &self,
        query: String
    ) -> Result<WeatherMultiple, Box<dyn Error>> {
        let addr = self.format_addr(query).await?;
        let response = self.get_response(&addr).await?;
        let data = self.parse_json_multiple(&response).await?;
        Ok(data)
    }

    pub async fn get_by_city(
        &self,
        city: &str
    ) -> Result<Weather, Box<dyn Error>> {
        self.get(format!("weather?q={}", &city)).await
    }


    pub async fn get_by_city_and_country(
        &self,
        city: &str,
        country: &str
    ) -> Result<Weather, Box<dyn Error>> {
        self.get(format!("weather?q={},{}", &city, &country)).await
    }

    pub async fn get_by_coordinates(
        &self,
        lat: f32,
        lon: f32
    ) -> Result<Weather, Box<dyn Error>> {
        self.get(format!("weather?lat={}&lon={}", lat, lon)).await
    }

    pub async fn get_by_zipcode(
        &self,
        zipcode: u32,
        country: &str,
    ) -> Result<Weather, Box<dyn Error>> {
        self.get(format!("weather?q={},{}", zipcode, country)).await
    }

    pub async fn get_by_cities_in_zone(
        &self,
        lon1: f32,
        lat1: f32,
        lon2: f32,
        lat2: f32,
        zoom: u32,
    ) -> Result<WeatherMultiple, Box<dyn Error>> {
        self.get_multiple(format!("box/city?bbox={},{},{},{},{}", lon1, lat1, lon2, lat2, zoom)).await
    }

    pub async fn get_by_cities_in_cycle(
        &self,
        lon: f32,
        lat: f32,
        count: u32,
    ) -> Result<WeatherMultiple, Box<dyn Error>> {
        self.get_multiple(format!("find?lat={}&lon={}&cnt={}", lon, lat, count)).await
    }

    async fn get(
        &self,
        query: String
    ) -> Result<Weather, Box<dyn Error>> {
        let addr = self.format_addr(query).await?;
        let response = self.get_response(&addr).await?;
        let data = self.parse_json(&response).await?;
        Ok(data)
    }

    async fn format_addr(
        &self,
        query: String
    ) -> Result<String, Box<dyn std::error::Error>> {
        let base_http = "https://api.openweathermap.org/data/2.5/";
        Ok(format!(
            "{}{}&appid={}&units={}",
            &base_http, &query, self.api_key, self.units.value()
        ))
    }

    async fn get_response(
        &self,
        addr: &str
    ) -> Result<String, reqwest::Error> {
        let json = reqwest::get(addr).await?.text().await?;
        Ok(json)
    }

    async fn parse_json(
        &self,
        json: &str
    ) -> Result<Weather, serde_json::error::Error> {
        let data: Weather = serde_json::from_str(&json.to_string())?;
        Ok(data)
    }

    async fn parse_json_multiple(
        &self,
        json: &str
    ) -> Result<WeatherMultiple, serde_json::error::Error> {
        let data: WeatherMultiple = serde_json::from_str(&json.to_string())?;
        Ok(data)
    }

}
