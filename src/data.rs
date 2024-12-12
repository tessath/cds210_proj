use crate::model::{DirtyHouseRecord, CleanHouseRecord}; 
use csv; 
use std::error::Error; 

pub fn process_csv(file_path: &str) -> Result<Vec<CleanHouseRecord>, Box<dyn Error>> {
    //creates CSV reader from file
    let mut rdr = csv::Reader::from_path(file_path)?; 
    //initializes empty vector
    let mut cleanv = Vec::new();  

    //iterates over each record and deserializes
    for result in rdr.deserialize::<DirtyHouseRecord>() {
        match result {
            Ok(record) => cleanv.push(crate::model::clean_csv(record)), 
            Err(err) => eprintln!("Error reading record: {}", err),
        }
    }

    //return cleaned record vector
    Ok(cleanv)
}

pub fn explore_data(records: &[CleanHouseRecord]) {
   // counts # of records 
    let count = records.len();  

    //if error, returns 0
    if count == 0 {
        println!("No data available for exploration.");
        return;
    }

    //calculates area and price of every record combined
    let total_area: f64 = records.iter().map(|r| r.area).sum(); 
    let total_price: u64 = records.iter().map(|r| r.price).sum(); 

    //calculates the avg area and price
    let avg_area = total_area / count as f64;  
    let avg_price = total_price as f64 / count as f64; 

    //calculate the max + min prices
    let max_price = records.iter().map(|r| r.price).max().unwrap(); 
    let min_price = records.iter().map(|r| r.price).min().unwrap(); 

    //prints stats
    println!("Data Stats");
    println!("____________");
    println!("Number of Records: {}", count); 
    println!("Average Area: {:.2}", avg_area); 
    println!("Average Price: {:.2}", avg_price); 
    println!("Max Price: {}", max_price);
    println!("Min Price: {}", min_price); 
}