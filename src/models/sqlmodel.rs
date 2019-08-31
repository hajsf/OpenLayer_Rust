#[derive(Serialize, Deserialize, Debug)]
pub struct SQLModel {
    pub trx_id: String,
    pub sql: String,
    pub updated: bool
}
