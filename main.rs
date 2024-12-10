use ndarray::Array2;  
use linfa_trees::DecisionTree;
use linfa::prelude::*;  
use std::error::Error;
use std::fs::File;  
use std::io::Write;
use data::{process_csv, explore_data};
use model::{CleanHouseRecord, DirtyHouseRecord};