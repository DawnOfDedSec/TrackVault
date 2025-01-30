use serde::{Deserialize, Serialize};
use std::{env, fs};

const CONFIG_FILE_PATH: &str = "src/res/config.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub host_id: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host_id: String::new(), // Default value for host_id
        }
    }
}

#[warn(dead_code)]
fn read_json() -> Result<Config, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join(CONFIG_FILE_PATH);

    let json_str = fs::read_to_string(file_path)?;
    let data: Config = serde_json::from_str(&json_str)?;
    Ok(data)
}

#[warn(dead_code)]
fn write_json(config_data: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join(CONFIG_FILE_PATH);

    let json_str = serde_json::to_string_pretty(config_data)?;
    fs::write(file_path, json_str)?;
    Ok(())
}

#[warn(dead_code)]
pub fn modify(key: &str, value: &str) {
    match read_json() {
        Ok(mut data) => {
            match key {
                "host_id" => data.host_id = value.to_string(),
                _ => println!("Invalid key: {}", key),
            }
            write_json(&data).expect("Failed to write config");
        }

        Err(err) => println!("Error reading config: {}", err),
    };
}

pub fn get(key: &str) {
    match read_json() {
        Ok(data) => match key {
            "host_id" => println!("Host ID: {}", data.host_id),
            _ => println!("Invalid key: {}", key),
        },
        Err(err) => println!("Error reading config: {}", err),
    }
}
