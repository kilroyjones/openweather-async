pub enum Units {
    Metric,
    Fahrenheit,
    Kelvin,
}

impl Units {
    pub fn value(&self) -> &str {
        match *self {
            Units::Metric => "metric",
            Units::Fahrenheit => "fahrenheit",
            Units::Kelvin => "kelvin",
        }
    }
}
