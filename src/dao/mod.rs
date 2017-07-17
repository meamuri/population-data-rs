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
            name: name.to_string(),
            year: year,
            value: val,
        }
    }

    pub fn get_country(&self) -> String { self.country.clone() }
    pub fn get_year(&self) -> i32 { self.year }
    pub fn get_city_name(&self) -> String { self.name.clone()  }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "country: {}, name: {}; cnt: {}", self.country, self.name, self.value)
    }
}