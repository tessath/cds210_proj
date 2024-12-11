use serde::Deserialize; 

#[derive(Debug, Deserialize)] 
pub struct DirtyHouseRecord {
    //csv columns used by decision tree
    pub area: f64,            
    pub bedrooms: u64,        
    pub bathrooms: f64,       
    pub floors: f64,          
    pub year_built: f64,      
    pub price: u64,           
}

#[derive(Debug, Default)] 
pub struct CleanHouseRecord {
    //corresponding csv cols
    pub area: f64,            
    pub bedrooms: u64,        
    pub bathrooms: f64,       
    pub floors: f64,          
    pub year_built: f64,      
    pub price: u64,           
}

pub fn clean_csv(r: DirtyHouseRecord) -> CleanHouseRecord {
    // takes a dirty data and returns cleaned version 
    CleanHouseRecord {
        area: r.area,         
        bedrooms: r.bedrooms,  
        bathrooms: r.bathrooms, 
        floors: r.floors,       
        year_built: r.year_built, 
        price: r.price,         
    }
}
