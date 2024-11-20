/// json_parse.rs: fetch database from an url

/// Contains: get_app_database_value(app, val): Fetch the database
/// (currently from a discord link from a conversation between me and .BRKT) 
/// and get the specified value.

/// get_preferences_value(category, pref): Fetch the preferences.json file from ~/.config/Apptrium
/// and get the specified value.

/// theridev was here
/// Nov 19: Apptrium-Legacy-ToRust
/// Nov 19: master
/// Nov 20: master

use serde_json::Value;
use serde::de::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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

// Example config: FOR TESTING PURPOSES: open the terminal
// mkdir ~/.config/Apptrium && cd ~/.config/Apptrium && wget 'https://cdn.discordapp.com/attachments/1292183239632486492/1308766395479097416/preferences.json?ex=673f2330&is=673dd1b0&hm=b33213cf2bb930d88c66f5e8643b0d570d72eb3e8f8b67019951580516179080&' -O preferences.json
pub fn get_preferences_value(category: &str, pref: &str) -> serde_json::Result<String> {
    // Get home dir
    let home_dir = dirs::home_dir().ok_or_else(|| {
        serde_json::Error::custom("Failed to retrieve home directory")
    })?;

    // Construct the path to preferences.json
    let config_path: PathBuf = home_dir.join(".config/Apptrium/preferences.json");

    // Open the preferences file
    let mut file = File::open(&config_path).map_err(|e| {
        serde_json::Error::custom(format!("Failed to open file: {}", e))
    })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| {
        serde_json::Error::custom(format!("Failed to read file: {}", e))
    })?;

    let v: Value = serde_json::from_str(&contents)?;

    if let Some(return_value) = v[category][0][pref].as_str() {
        Ok(return_value.to_string()) // Return the found value as a String
    } else {
        Err(serde_json::Error::custom(format!(
            "Value not found for category '{}' and preference '{}'. This incident will be reported.", category, pref
        )))
    }
}

