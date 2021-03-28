pub mod openweather;
pub mod models;
pub mod units;

pub use openweather::OpenWeather;
pub use models::*;
pub use units::Units;

#[cfg(test)]
mod test {
    use super::openweather::{OpenWeather, Units};
    use std::env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn get_by_city() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric);
        let _temp = weather.get_by_city("Tokyo").await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_by_city_and_country() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric);
        let _temp = weather.get_by_city_and_country("Tokyo", "Japan").await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_by_coordinates() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric);
        let _temp = weather.get_by_coordinates(56.0, 12.0).await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_by_zipcode() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric);
        let _temp = weather.get_by_zipcode(80918, "US").await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_by_cities_in_zone() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric);
        weather.get_by_cities_in_zone(12.0, 32.0, 15.0, 37.0, 10).await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_by_cities_in_cycle() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric);
        weather.get_by_cities_in_cycle(12.0, 32.0, 3).await?;
        Ok(())
    }

}
