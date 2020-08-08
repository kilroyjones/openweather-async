pub use crate::openweather_types::*;
pub use crate::location::*;

static API_KEY: &str = "8474e8cb48a479e77a7d5ed429fc4cf2";

async fn get_response(addr: &str) -> Result<String, reqwest::Error> {
    let json = reqwest::get(addr)
        .await?
        .text()
        .await?;
    Ok(json)
}

async fn parse_json(json: &str) -> Result<OpenWeather, serde_json::error::Error> {
    let data: OpenWeather = serde_json::from_str(&json.to_string())?;
    Ok(data)
}

async fn format_addr(query: String, units: &str) -> Result<String, Box<dyn std::error::Error>> {
    let base_http = "https://api.openweathermap.org/data/2.5/weather?";
    Ok(
        format!("{}{}&appid={}&units={}",
            &base_http,
            &query,
            API_KEY,
            &units)
    )
}

pub struct OpenWeatherRequest<'a> {
    pub location: Location<'a>,
    pub units: Units
}


impl OpenWeather {
    pub async fn get(request: OpenWeatherRequest<'_>) -> Result<OpenWeather, Box<dyn std::error::Error>> {
        let prefix = "q=".to_string();
        let addr = match request.location {
            Location::City(city) =>
                format_addr(prefix + &city.to_string(), request.units.value()).await?,
            Location::CityAndCountry(city, country) =>
                format_addr(prefix + &city.to_string() + &",".to_string() + &country.to_string(), request.units.value()).await?,
            Location::LatitudeAndLongitude(lat, lon) =>
                format_addr(format!("lat={}&lon={}", lat, lon), request.units.value()).await?,
            Location::ZipCode(zip_code) =>
                format_addr(format!("{}{}", prefix, zip_code), request.units.value()).await?,
        };
        let response = get_response(&addr).await?;
        let data = parse_json(&response).await?;
        Ok(data)
    }
}


