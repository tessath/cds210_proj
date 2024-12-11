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
            Err(err) => println!("{}", err),
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
        flat_values.push(record.area);
        flat_values.push(record.bedrooms as f64);
        flat_values.push(record.bathrooms);
        flat_values.push(record.floors);
        flat_values.push(record.year_built);
        labels.push((record.price > 500000) as usize); 
    }


    let array = Array2::from_shape_vec((cleanv.len(), 5), flat_values)
    .expect("Error creating ndarray");
let labels_array = Array2::from_shape_vec((labels.len(), 1), labels)
    .expect("Error creating labels array");


let dataset = Dataset::new(array, labels_array.column(0).to_owned())
    .with_feature_names(vec!["area", "bedrooms", "bathrooms", "floors", "year_built"]);
    let decision_tree = DecisionTree::params()
        .max_depth(Some(4))
        .fit(&dataset)
        .expect("Failed to train decision tree");


    let pred = decision_tree.predict(&dataset);
    let cm = pred.confusion_matrix(&dataset).expect("Failed to compute confusion matrix");
    println!("Accuracy: {:?}", cm.accuracy());


    Ok(())
}


