/// json_parse.rs: fetch database from an url
/// Contains: get_app_database_value(app, val): Fetch the database
/// (currently from a discord link from a conversation between me and .BRKT) 
/// and get the specified value.

/// theridev was here
/// Nov 19: Apptrium-Legacy-ToRust

use serde_json::Value;
use serde::de::Error;

#[allow(dead_code)]
pub fn get_app_database_value(app: &str, val: &str) -> serde_json::Result<String> {
    // Fetch data from the URL
    let contents = reqwest::blocking::get("https://cdn.discordapp.com/attachments/1292183239632486492/1308441965821694012/apps.json?ex=673df50a&is=673ca38a&hm=d15eeaa144d1d949f28dac59d46357a60c045c6624ec8044d2ee4ee77e69a5d8&")
        .map_err(|e| serde_json::Error::custom(format!("HTTP request failed: {}", e)))?
        .text()
        .map_err(|e| serde_json::Error::custom(format!("Failed to get text: {}", e)))?;

    // Parse the JSON content
    let v: Value = serde_json::from_str(&contents)?;

    // Extract the required value
    if let Some(return_value) = v[app][0][val].as_str() {
        Ok(return_value.to_string()) // Return the found value as a String
    } else {
        Err(serde_json::Error::custom(format!(
            "Value not found for app '{}' and key '{}'", app, val
        )))
    }
}
