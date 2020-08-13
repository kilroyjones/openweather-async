# openweather-async


This rust library is used to access the **Current Weather** portion of the [OpenWeather API](https://openweathermap.org/). It is not an official library, and was created with reference to a synchronous version found [here](https://github.com/BroderickCarlin/openweather).

## setup

To use this library you will need to first sign up at [OpenWeather API Sign up](https://openweathermap.org/home/sign_up) and get an API key. If interested, the API documentation is [here](https://openweathermap.org/current) with examples of the json-formatted response. 

To follow the example below, you should store the API key in the **.env** file as **OPENWEATHER_API_KEY=YOUR_API_KEY**.  The **.env** file should be place in the root folder of your project. 

```
├── Cargo.toml
├── .env
├── src
│   ├── main.rs

```

Next, you'll need to add the following dependencies to your **Cargo.toml**.

```
openweather-async = "0.0.1"
tokio = { version="0.2.22", features = ["macros", "tcp", "dns", "io-util"]}
dotenv = "0.15.0"
```

## example program

The program below will retrieve the full weather report represented as the **OpenWeather** type. This type, and the other types that make it up, can be seen found in the **models.rs** file. If you're looking for a particular field it will help to look there or in the documentation. 

While you can directly pass the API key into **new**, it's recommended that you follow the example below and use the **dotenv** crate as shown in the example and mentioned above. 

```
use tokio;
use std::env;
use dotenv::dotenv;
use openweather_async::{ OpenWeather, Units};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("No env file found");
    let token = env::var("OPENWEATHER_API_KEY").unwrap();
    let weather: OpenWeather = OpenWeather::new(&token, Units::Metric).await?;
    let report = weather.get_by_city("Tokyo").await?;
    println!("{:?}", report);
    println!("{:?}", report.main.weather);
    println!("{:?}", report.weather.description);
    println!("{:?}", report.wind.speed);
   Ok(())
}
```

There are six different ways you can access the API, with the last two  can also access other functions as shown below. Keep in mind that each of these calls will be a new request to the API. If you get by the city name you can find the country code within the returned **OpenWeather** struct.

```
weather.get_by_city_and_country("Tokyo", "Japan").await?;
weather.get_by_coordinates(56.0, 12.0).await?;
weather.get_by_zipcode(80918, "US").await?;
weather.get_by_cities_in_zone(12.0, 32.0, 15.0, 37.0, 10).await?;
weather.get_by_cities_in_cycle(12.0, 32.0, 3).await?;
```
