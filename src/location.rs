
pub enum Location<'a> {
    City (&'a str),
    CityAndCountry (&'a str, &'a str ),
    LatitudeAndLongitude (f32, f32),
    ZipCode (u32)
}


pub enum Units {
    Metric,
    Fahrenheit,
    Kelvin
}

impl Units {
    pub fn value(&self) -> &str {
        match *self {
            Units::Metric => "metric",
            Units::Fahrenheit => "fahrenheit",
            Units::Kelvin => "kelvin"
        }
    }
}



