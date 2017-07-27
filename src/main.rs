extern crate population_data_rs;

use std::env;
use population_data_rs::app::{app_millionaires, app_population, app_ratio, app_top};

fn main() {
    let args: Vec<String> = env::args().collect();    
    if args.len() < 2 {
        println!("запустите программу с одним из аргументов командной строки:");
        println!("millionaires population ratio top");
        return;
    }

    match &args[1][..] {
        "millionaires" => app_millionaires(),
        "population" => app_population(),
        "ratio" => app_ratio(),
        "top" => app_top(),
        _ => println!("incorrect args!"),
    }
}
