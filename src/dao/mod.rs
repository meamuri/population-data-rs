use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    country: String,
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing 
    name: String,
    year: i32,
    value: f64,
    sex: char,
}

impl City {
    pub fn new(country: &str, name: &str, year: i32, val: f64, sex: &str) -> City {
        let letter = match sex {
            "Male" => 'm',
            "Female" => 'f',
            _ => 'b',
        };
        City {
            country: country.to_string(),
            name: name.to_string(),
            year: year,
            value: val,
            sex: letter,
        }
    }

    pub fn get_population(&self) -> f64 { self.value }
    pub fn get_country(&self) -> String { self.country.clone() }
    pub fn get_year(&self) -> i32 { self.year }
    pub fn get_city_name(&self) -> String { self.name.clone()  }
    pub fn get_part(&self) -> char { self.sex }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "country: {}, name: {}; cnt: {}, year: {}, part: {}", 
        self.country, self.name, self.value, self.year, self.sex )
    }
}

impl Clone for City {
    fn clone(&self) -> City {
        City {
            country: self.country.clone(),
            name: self.name.clone(),
            ..*self
        }
    }
}