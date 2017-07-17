extern crate csv;

use std::fs::File;
use std::error::Error;
use std::path::Path;

pub fn read_csv(file_path: &str)  -> Result<(), Box<Error>> {    
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())    
}