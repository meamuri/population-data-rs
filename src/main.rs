extern crate population_data_rs;

use population_data_rs::computing::loader::read_csv;

use std::process;

fn main() {
    let file_path = String::from("data/unsd-citypopulation-year-both.csv");
    if let Err(err) = read_csv(&file_path[..]) {
        println!("{}", err);
        process::exit(1);
    }
    
}
