use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

#[derive(Debug, Deserialize)]
struct KeyLightState {
    on: i32,
}

#[derive(Debug, Serialize)]
struct KeyLightToggle {
    on: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    
    // Replace with the actual URL of your Elgato Key Light API
    let url = "http://192.168.1.105:9123/elgato/lights";
    
    // Send a GET request to retrieve the current state
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    let json_body: Value = serde_json::from_str(&body)?;
    
    let current_state: KeyLightState = serde_json::from_value(json_body["lights"][0].clone())?;
    println!("Current state: {:?}", current_state);
    
    // Toggle the state
    let new_state = KeyLightToggle {
        on: if current_state.on == 1 { 0 } else { 1 },
    };
    
    let payload = json!({
        "numberOfLights": 1,
        "lights": [new_state]
    });
    
    // Send a PUT request to update the state
    let _response = client
        .put(url)
        .json(&payload)
        .send()
        .await?;
    
    Ok(())
}

