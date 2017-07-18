use std::collections::HashMap;
use super::super::dao::City;

pub fn millionaires(countries: &HashMap<String, Vec<City>>, level: f64) -> HashMap<String, i32> {
    let mut res = HashMap::new();
    for (country, cities) in countries {
        let count = res.entry(country.clone()).or_insert(0);
        for city in cities.iter() {
            *count += if city.get_population() > level { 1 } else { 0 };
        }
    }
    res
}

pub fn population(countries: HashMap<String, Vec<City>>) -> HashMap<String, f64> {
    let res = HashMap::new();
    res
}

pub fn top(countries: HashMap<String, Vec<City>>) -> HashMap<String, Vec<City>> {
    let res = HashMap::new();
    res
}

