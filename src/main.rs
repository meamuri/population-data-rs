extern crate population_data_rs;

use population_data_rs::computing::loader::{read_csv, combine_by_countries, select_useful};
use population_data_rs::computing::solve::{millionaires, population, top};
use population_data_rs::computing::saver::{save_millionaires, save_top};

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
    // this func works!
    // сошлись:
    // chine - 159
    // Australia - 10 
    println!("Стран: {}", res_millionaires.len());
    for (key, val) in  &res_millionaires {
        println!("{} : {}", key, val);        
    }
    
    let res_population = population(&countries);
    // this func works!
    // стран по-прежнему 208
    println!("\n\n\nСтран: {}", res_population.len());
    for (key, val) in &res_population {
        println!("{} : {}", key, val);
    }

    let res_top = top(&countries, TOP_N);
    // this func works!
    // стран по-прежнему 208
    println!("\n\n\nСтран: {}", res_top.len());
    for (key, val) in &res_top {
        println!("{} : ", key);
        for it in val {
            println!("\t : {} ", it);
        }
    }

    save_millionaires(&res_millionaires);
    save_top(&res_top);
}
