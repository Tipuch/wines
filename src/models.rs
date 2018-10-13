#[derive(Queryable)]
pub struct SaqWine {
    pub id: u32,
    pub name: String,
    pub country: String,
    pub region: String,
    pub designation_of_origin: String,
    pub regulated_designation: bool,
    pub producer: String,
    // in liters
    pub volume: f64,
    pub alcohol_percent: u32,
    
}