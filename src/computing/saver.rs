use std::collections::HashMap;

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

// #[macro_use(bson, doc)]


pub fn save_millionaires(data: &HashMap<String, i32>) {
    let client = Client::connect("localhost", 27017)
        .ok().expect("Failed to initialize client.");
 
    
    let coll = client.db("dsr_rs").collection("millionaires");

    // Remove all documents.    
    coll.delete_many(doc!{}, None)
        .ok().expect("Failed to delete documents.");

    let mut records = Vec::new();
    for (country, count_millionaires) in data {        
        let header = country.clone();
        let info = *count_millionaires;
        let doc = doc!{
            "country" => header, 
            "count_millionaires" => info
            };
        records.push(doc)
    }
    
    // Insert multiple documents with default options.
    coll.insert_many(records, None)
        .ok().expect("Failed to insert documents.");

}