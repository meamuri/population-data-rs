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

pub fn population(countries: &HashMap<String, Vec<City>>) -> HashMap<String, f64> {
    
    let mut res = HashMap::new();
    for (country, cities) in countries {
        let population = res.entry(country.clone()).or_insert(0.0);
        for city in cities.iter() {
            *population += city.get_population();
        }
    }
    res
}

pub fn top(countries: &HashMap<String, Vec<City>>, n: usize) -> HashMap<String, Vec<City>> {
    let mut res = HashMap::new();
    for (country, cities) in countries {
        let mut cities = cities.clone();
        let top = res.entry(country.clone()).or_insert(Vec::new());
        cities.sort_by(|a, b| b.get_population().partial_cmp(&a.get_population()).unwrap());
        for city in cities.iter().take(n) {
            top.push(city.clone());
        }
    }
    res
}

