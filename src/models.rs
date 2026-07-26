use serde::Serialize;

#[derive(Serialize)]
pub struct PortInfo {
    pub port: u16,
    pub status: String,
    pub service: String,
}
