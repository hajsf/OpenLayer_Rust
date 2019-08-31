#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct Task {
    pub command: String,
    pub truck_id: String,
    pub task: String,
    pub details: String,
    pub lon: f32,
    pub lat: f32,
    pub address: String,
    pub date_picker: String,
    pub time_picker: String,
    pub status: String,
    pub customer: String,
    pub mobile: usize
}
