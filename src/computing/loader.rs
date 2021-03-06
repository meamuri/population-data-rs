extern crate csv;

use super::super::dao::City;

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
        res.push(City::new(
            &record[0],
            &record[4],
            *&record[1].parse::<i32>().unwrap_or_default(),
            *&record[9].parse::<f64>().unwrap_or_default(),
            &record[3]
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
        if city.get_year() < year {
            *city = val.clone();
        }        
    }    
    
    let mut res = Vec::new();
    for (_, val) in checker.iter() {
        res.push(val.clone())
    }
    res
}

pub fn select_useful_diff(records: Vec<City>) -> Vec<City> {
    let f_records: Vec<City> = records.iter()
        .filter(|city| city.get_part() == 'f')
        .map(|city| city.clone())
        .collect();
    let m_records: Vec<City> = records.iter()
        .filter(|city| city.get_part() == 'm')
        .map(|city| city.clone())
        .collect();
    let mut f_records = select_useful(f_records);
    let mut m_records = select_useful(m_records);    
    f_records.append(&mut m_records);
    f_records
}

pub fn combine_by_countries(data: Vec<City>) -> HashMap<String,Vec<City>> {
    let mut res = HashMap::new();
    for val in data.iter() {
        let vector = res.entry(val.get_country()).or_insert(Vec::new());
        vector.push(val.clone());
    }    
    res
}