use ndarray::{Array2};
use linfa_trees::DecisionTree;
use linfa::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;




#[derive(Debug, Deserialize)]
struct DirtyHouseRecord {
    area: f64,
    bedrooms: u64,
    bathrooms: f64,
    floors: f64,
    year_built: f64,
    price: u64,
}




#[derive(Debug, Default)]
struct CleanHouseRecord {
    area: f64,
    bedrooms: u64,
    bathrooms: f64,
    floors: f64,
    year_built: f64,
    price: u64,
}




fn clean_csv(r: DirtyHouseRecord) -> CleanHouseRecord {
    CleanHouseRecord {
        area: r.area,
        bedrooms: r.bedrooms,
        bathrooms: r.bathrooms,
        floors: r.floors,
        year_built: r.year_built,
        price: r.price,
    }
}




fn process_csv(file_path: &str) -> Result<Vec<CleanHouseRecord>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut cleanv = Vec::new();




    for result in rdr.deserialize::<DirtyHouseRecord>() {
        match result {
            Ok(record) => cleanv.push(clean_csv(record)),
            Err(err) => eprintln!("Error reading record: {}", err),
        }
    }




    Ok(cleanv)
}


fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "C:/Users/tessa/ds210_proj/House Price Prediction Dataset.csv";
    let cleanv = process_csv(file_path)?;
    let mut flat_values: Vec<f64> = Vec::new();
    let mut labels: Vec<usize> = Vec::new();


    for record in &cleanv {
        flat_values.extend_from_slice(&[
            record.area,
            record.bedrooms as f64,
            record.bathrooms,
            record.floors,
            record.year_built,
        ]);
        labels.push((record.price > 500_000) as usize);
    }


    let array = Array2::from_shape_vec((cleanv.len(), 5), flat_values)
        .map_err(|_| "Failed to create feature matrix from the input data")?;
    let labels_array = Array2::from_shape_vec((labels.len(), 1), labels)
        .map_err(|_| "Failed to create label array from the input data")?;


    let dataset = Dataset::new(array, labels_array.column(0).to_owned())
        .with_feature_names(vec!["area", "bedrooms", "bathrooms", "floors", "year_built"]);


    let decision_tree = DecisionTree::params()
        .max_depth(Some(20))
        .fit(&dataset)?;


    let pred = decision_tree.predict(&dataset);
    let cm = pred.confusion_matrix(&dataset)?;
    println!("Accuracy: {:?}", cm.accuracy());


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










