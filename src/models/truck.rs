use ws::Sender;
#[derive(Debug)]
pub struct Truck {
    pub socket: Sender,
    pub unique_id: String
}
