use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub client_id: String,
    pub client_secret: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    pub artists: Vec<String>,
    pub name: String,
    pub release_date: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub id: String
}