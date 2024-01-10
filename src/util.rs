use std::fs::File;
use std::io::Read;
use reqwest::{Client, Error};
use serde_json::Value;
use crate::data_structures::ClientData;

// There are 60,000 milliseconds in a minute.
// To round to the nearest minute, add half of this value before dividing.
pub fn ms_to_minutes(ms: i64) -> i64 {
    (ms + 30000) / 60000
}

pub fn str_to_field(json: String, key: &str) -> String {
    val_to_str(&serde_json::from_str(&json).unwrap(), key)
}

pub fn val_to_str(json: &Value, key: &str) -> String {
    let value = &mut String::new();

    // Extract the client_id value.
    if let Some(client_id) = json[key].as_str() {
        *value = client_id.to_owned();
    } else {
        panic!("Field \'{}\' not found!", key);
    }
    value.to_string()
}

pub async fn get_token(client: &Client, client_data: &ClientData) -> Result<String, Error> {
    let response = client.post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[("client_id", &client_data.client_id), ("client_secret", &client_data.client_secret),("grant_type", &"client_credentials".to_owned())])
        .send().await?;

    Ok(response.text().await?)
}

pub fn client_data() -> ClientData {
    let file_path = "web_data.json";
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    serde_json::from_value(serde_json::from_str(&contents).expect("Err 1")).expect("Err 2")
}