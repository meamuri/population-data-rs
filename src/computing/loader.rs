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