mod openweather;
mod openweather_types;
mod location;

#[cfg(test)]
mod test {
    use super::openweather::{OpenWeather, OpenWeatherRequest, Location, Units};

    #[tokio::test]
    async fn by_city() -> Result<(), Box<dyn std::error::Error>>  {
        let request: OpenWeatherRequest = OpenWeatherRequest{
            location: Location::City("Tokyo"),
            units: Units::Metric };
        let weather: OpenWeather = OpenWeather::get(request).await?;
        Ok(())
    }

    #[tokio::test]
    async fn by_city_and_country() -> Result<(), Box<dyn std::error::Error>>  {
        let request: OpenWeatherRequest = OpenWeatherRequest{ 
            location: Location::CityAndCountry("Taipei", "Taiwan"),
            units: Units::Metric };
        let weather: OpenWeather = OpenWeather::get(request).await?;
        Ok(())
    }

    #[tokio::test]
    async fn by_latitude_and_longitude() -> Result<(), Box<dyn std::error::Error>>  {
        let request: OpenWeatherRequest = OpenWeatherRequest{ 
            location: Location::LatitudeAndLongitude(10.0, 14.5),
            units: Units::Metric };
        let weather: OpenWeather = OpenWeather::get(request).await?;
        Ok(())
    }

    #[tokio::test]
    async fn by_zip_code() -> Result<(), Box<dyn std::error::Error>>  {
        let request: OpenWeatherRequest = OpenWeatherRequest{ 
            location: Location::ZipCode(94107),
            units: Units::Metric };
        let weather: OpenWeather = OpenWeather::get(request).await?;
        Ok(())
    }

}
