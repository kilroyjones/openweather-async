mod units;
mod openweather;
mod models;

#[cfg(test)]
mod test {
    use super::openweather::{OpenWeather, Units};
    use std::env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn get_by_city() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
        let temp = weather.get_by_city("Tokyo").await?;
        println!("{:?}", temp.main.temp);
        Ok(())
    }

    #[tokio::test]
    async fn get_by_city_and_country() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
        let temp = weather.get_by_city_and_country("Tokyo", "Japan").await?;
        println!("{:?}", temp.main.temp);
        Ok(())
    }

    #[tokio::test]
    async fn get_by_coordinates() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
        let temp = weather.get_by_coordinates(56.0, 12.0).await?;
        println!("{:?}", temp.main.temp);
        Ok(())
    }

    #[tokio::test]
    async fn get_by_zipcode() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
        let temp = weather.get_by_zipcode(80918, "US").await?;
        println!("{:?}", temp.main.temp);
        Ok(())
    }

    #[tokio::test]
    async fn get_by_cities_in_zone() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("No env file found");
        let token = env::var("OPENWEATHER_API_KEY").unwrap();
        let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
        weather.get_by_cities_in_zone(12.0, 32.0, 15.0, 37.0, 10).await?;
        //println!("{:?}", temp);
        Ok(())
    }



}
