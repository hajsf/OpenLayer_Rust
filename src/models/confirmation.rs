#[derive(Serialize, Deserialize)]
pub struct Confirmation {
    pub command: String,
    pub trx_id: String,
    pub updated: bool
}
