mod model; 
mod data; 

use ndarray::Array2;  
use linfa_trees::DecisionTree; 
use linfa::prelude::*;  
use std::error::Error; 
use std::fs::File;  
use std::io::Write; 
use data::{process_csv, explore_data}; 
use model::{CleanHouseRecord, DirtyHouseRecord}; 

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "C:/Users/tessa/cds210_proj/House Price Prediction Dataset.csv";
    //cleans csv using function defined in data.rs
    let cleanv = process_csv(file_path)?;
    //explores some basic stats of data
    explore_data(&cleanv);
    //prints first three records
    println!("First three records ex:");
    for record in cleanv.iter().take(3) {
        println!("{:?}", record);
    }

    //vectors to hold flattened feature values + labels
    let mut flat_values: Vec<f64> = Vec::new(); 
    let mut labels: Vec<usize> = Vec::new(); 
    // iterates over cleaned data, extracts features, and labels
    for record in &cleanv {
        flat_values.extend_from_slice(&[  
            record.area,
            record.bedrooms as f64,
            record.bathrooms,
            record.floors,
            record.year_built,
        ]);
        //if the price > 500,000 labeled as 1, otherwise 0
        labels.push((record.price > 500_000) as usize);
    }

    //converts to 2D array (error if not)
    let array = Array2::from_shape_vec((cleanv.len(), 5), flat_values)
        .map_err(|_| "Failed to create feature matrix from the input data")?; 

    let labels_array = Array2::from_shape_vec((labels.len(), 1), labels)
        .map_err(|_| "Failed to create label array from the input data")?; 

    //makes dataset object with features, labels, and assigns names
    let dataset = Dataset::new(array, labels_array.column(0).to_owned())
    .with_feature_names(vec!["area", "bedrooms", "bathrooms", "floors", "year_built"]);

    //initializes + train a decision tree
    let decision_tree = DecisionTree::params()
        .max_depth(Some(20))
        .fit(&dataset)?; 

    let pred = decision_tree.predict(&dataset);
    
    //confusion matrix needed to evaluate model
    let cm = pred.confusion_matrix(&dataset)?;
    println!("Results");
    println!("________");
    println!("Accuracy: {:?}", cm.accuracy());

    //saves visualization to a tikz file for LaTeX
    let mut tikz_file = File::create("decision_tree_visual.tex")
        .map_err(|_| "Failed to create output TikZ file")?; 
    tikz_file.write_all(
        decision_tree
            .export_to_tikz() 
            .with_legend()  
            .to_string()    
            .as_bytes(),
    )?;  
    
    println!("TikZ visualization saved as 'decision_tree_visual.tex'. Compile it with LaTeX!");

    Ok(()) 
}

#[cfg(test)]  
mod tests {
    use super::*; 
    use crate::data::process_csv; 
    use crate::model::CleanHouseRecord; 

    #[test] //tests process_csv func
    fn test_process_csv() {
        let file_path = "C:/Users/tessa/cds210_proj/House Price Prediction Dataset.csv";
        let result = process_csv(file_path);  
        assert!(result.is_ok());  
        let records = result.unwrap();  
        assert!(!records.is_empty());  
    }

    #[test]
    fn test_clean_csv() {
        //tests cleaned csv for expected vals
        use crate::model::DirtyHouseRecord;  
        
        //sample
        let dirty_record = DirtyHouseRecord {
            area: 1200.0,
            bedrooms: 3,
            bathrooms: 2.0,
            floors: 1.0,
            year_built: 2000.0,
            price: 250_000,
        };
        
        let clean_record = crate::model::clean_csv(dirty_record);
        
        assert_eq!(clean_record.area, 1200.0);
        assert_eq!(clean_record.bedrooms, 3);
        assert_eq!(clean_record.bathrooms, 2.0);
        assert_eq!(clean_record.floors, 1.0);
        assert_eq!(clean_record.year_built, 2000.0);
        assert_eq!(clean_record.price, 250_000);
    }
}