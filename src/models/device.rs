use uuid::Uuid;

pub struct Device {
    pub id: Uuid,
    pub data: serde_json::Value
}
