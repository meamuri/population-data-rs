extern crate population_data_rs;

use population_data_rs::computing::loader::{read_csv, combine_by_countries, select_useful};
use population_data_rs::computing::solve::{millionaires, population, top};
use population_data_rs::computing::saver::{save_millionaires, save_top, save_population};

const POPULATION_LEVEL: f64 = 1_000_000.0;
const TOP_N: usize = 5;

fn main() {
    let file_path = String::from("data/unsd-citypopulation-year-both.csv");
    
    // records: 17059
    let records = match read_csv(&file_path){
        Ok(r) => r,
        Err(_) => { panic!("ooops") },
    };
    
    let cities = select_useful(records);                // cities: 4501
    let countries = combine_by_countries(cities);       // countries: 208

    
    let res_millionaires = millionaires(&countries, POPULATION_LEVEL);
    let res_population = population(&countries);    
    let res_top = top(&countries, TOP_N);
    
    save_millionaires(&res_millionaires);
    save_top(&res_top);
    save_population(&res_population);
}
