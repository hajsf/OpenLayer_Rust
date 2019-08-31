#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub unique_id: String,
    pub lat: f64,
    pub lon: f64,
    pub speed: f64,
    pub free_op: bool
}
