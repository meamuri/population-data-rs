extern crate population_data_rs;

use population_data_rs::computing::loader::{read_csv, combine_by_countries};
use population_data_rs::computing::loader::select_useful;

fn main() {
    let file_path = String::from("data/unsd-citypopulation-year-both.csv");
    
    // records: 17059
    let records = match read_csv(&file_path){
        Ok(r) => r,
        Err(_) => { panic!("ooops") },
    };

    // cities: 4501
    let cities = select_useful(records);

    // countries: 208
    let countries = combine_by_countries(cities);
    println!("{}", countries.len());

    // for val in cities.iter().take(30) {
    //     println!("{}", val);        
    // }
}
