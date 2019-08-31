#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub checkbox: bool,
    pub number: usize,
    pub radio: String,
    pub password: String,
    pub text_area: String,
    pub select: String,
}
