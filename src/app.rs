use computing::loader::{read_csv, combine_by_countries, select_useful,select_useful_diff};
use computing::solve::{millionaires, population, ratio, top};
use computing::saver::{save_millionaires, save_top, save_population, save_ratio};

const POPULATION_LEVEL: f64 = 1_000_000.0;
const TOP_N: usize = 5;
const FILE_BOTH: &'static str = "data/unsd-citypopulation-year-both.csv";
const FILE_DIFF: &'static str = "data/unsd-citypopulation-year-fm.csv";  

pub fn app_millionaires() {
    let file_path = String::from(FILE_BOTH);
    
    // records: 17059
    let records = match read_csv(&file_path){
        Ok(r) => r,
        Err(_) => { panic!("ooops") },
    };
    
    let cities = select_useful(records);                // cities: 4501
    let countries = combine_by_countries(cities);       // countries: 208

    let res_millionaires = millionaires(&countries, POPULATION_LEVEL);
    save_millionaires(&res_millionaires);    
}

pub fn app_population() {
    let file_path = String::from(FILE_BOTH);
    
    // records: 17059
    let records = match read_csv(&file_path){
        Ok(r) => r,
        Err(_) => { panic!("ooops") },
    };
    
    let cities = select_useful(records);                // cities: 4501
    let countries = combine_by_countries(cities);       // countries: 208

    let res_population = population(&countries);    
    save_population(&res_population);
}

pub fn app_ratio() {
    let file_path = String::from(FILE_DIFF);
    
    // records: 17059
    let records = match read_csv(&file_path){
        Ok(r) => r,
        Err(_) => { panic!("ooops") },
    };
    
    let records = select_useful_diff(records);
    
    let countries = ratio(&records);
    for (key, val) in &countries {
        println!("{}", key);
    }
    println!("{}", countries.len());
    save_ratio(&countries);
    // for val in &records {
    //     println!("{}", val);
    // }
    // println!("{}", records.len());

    // let cities = select_useful(records);                // cities: 4501
    // let countries = combine_by_countries(cities);       // countries: 208

    // for (key, val) in countries.iter.take(5) {
    //     println!(&key);
    // }
}

pub fn app_top() {
    let file_path = String::from(FILE_BOTH);
    
    // records: 17059
    let records = match read_csv(&file_path){
        Ok(r) => r,
        Err(_) => { panic!("ooops") },
    };
    
    let cities = select_useful(records);                // cities: 4501
    let countries = combine_by_countries(cities);       // countries: 208

    let res_top = top(&countries, TOP_N);
    save_top(&res_top);    
}