use std::fmt;

pub struct City {
    country: String,
    name: String,
    year: i32,
    value: f64
}

impl City {
    pub fn new(country: &str, name: &str, year: i32, val: f64 ) -> City {
        City {
            country: country.to_string(),
            name: country.to_string(),
            year: year,
            value: val,
        }
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "country: {}, name: {}; cnt: {}", self.country, self.name, self.value)
    }
}