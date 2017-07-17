extern crate csv;

use super::super::dao::*;

use std::fs::File;
use std::error::Error;
use std::path::Path;

use std::collections::HashMap;

pub fn read_csv(file_path: &str)  -> Result<Vec<City>, Box<Error>> {    
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut res = Vec::new();

    for result in rdr.records() {
        let record = result?;
        // res.push(City{
        //     country: String::from(&record[0]),
        //     name: String::from(&record[4]),
        //     year: *&record[1].parse::<i32>().unwrap_or_default(),
        //     value: *&record[9].parse::<f64>().unwrap_or_default()
        // });        
        res.push(City::new(
            &record[0],
            &record[4],
            *&record[1].parse::<i32>().unwrap_or_default(),
            *&record[9].parse::<f64>().unwrap_or_default()
        ));        
    }

    Ok(res)    
}

pub fn select_useful(data: Vec<City>) -> Vec<City> {
    let mut checker: HashMap<String, City> = HashMap::new();
    
    for val in data.iter() {
        let city_name = val.get_city_name();
        let year = val.get_year();

        let city = checker.entry(city_name.clone()).or_insert(val.clone());
        if (city.get_year() < year) {
            *city = val.clone();
        }

        
        // if !checker.contains_key(&city_name) || 
        //     checker.get(&city_name).get_year() < val.get_year() {
        //     checker.insert(city_name.clone(), val.clone());
        // }
        // let city = checker.entry(city_name.clone()).or_insert(val.clone());
        // if city.get_year() < val.get_year() {
        //     checker.insert(city_name.clone(), val.clone());
        // }
    }    
    
    let mut res = Vec::new();
    for (_, val) in checker.iter() {
        res.push(val.clone())
    }
    res
}

// pub fn select_useful(data: Vec<City>) -> HashMap<String,City> {
//     let mut res = HashMap::new();
//     for val in data.iter() {
//         res.insert(val.country.clone(), City::new{
//             country: val.country.clone(),
//             name: val.name.clone(),
//             year: val.year,
//             value: val.value
//         });
//     }    
//     res
// }