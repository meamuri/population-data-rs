extern crate population_data_rs;

use population_data_rs::computing::loader::read_csv;
use population_data_rs::computing::loader::select_useful;

fn main() {
    let file_path = String::from("data/unsd-citypopulation-year-both.csv");
    let records = match read_csv(&file_path){
        Ok(r) => r,
        Err(_) => { panic!("ooops") },
    };

    println!("{}", records.len());
    for val in records.iter().take(5) {
        println!("{}", val);        
    }

    let countries = select_useful(records);
    println!("{}", countries.len());
    // for (key, val) in &countries {
    //     println!("{}", key);        
    // }
}
