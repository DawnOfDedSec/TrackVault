use serde::{Deserialize, Serialize};
use std::{env, fs};

/// The line `const CONFIG_JSON_FILE: &str = "src/res/config.json";` is declaring a constant named
/// `CONFIG_JSON_FILE` with a type reference to a string slice (`&str`). The constant is assigned the
/// value `"src/res/config.json"`, which represents the file path to the JSON configuration file used in
/// the Rust code. This constant is used to specify the location of the configuration file that the code
/// will read from and write to.
const CONFIG_JSON_FILE: &str = "src/res/config.json";

#[derive(Debug, Deserialize, Serialize)]
/// The `Config` struct in Rust contains a field `host_id` of type `String`.
///
/// Properties:
///
/// * `host_id`: The `host_id` property in the `Config` struct represents the identifier for the host.
/// It is a string type, which means it stores textual data.
pub struct Config {
    pub host_id: String,
}

/// The `impl Default for Config` block in Rust is implementing the `Default` trait for the `Config`
/// struct. By doing this, it allows instances of the `Config` struct to be created with default values
/// without explicitly specifying them.
impl Default for Config {
    fn default() -> Self {
        Config {
            host_id: String::new(), // Default value for host_id
        }
    }
}

/// The function `read_json` reads a JSON file, deserializes it into a `Config` struct using Serde, and
/// returns a Result.
///
/// Returns:
///
/// The `read_json` function is returning a `Result` containing either a `Config` struct or a boxed `dyn
/// std::error::Error` trait.
#[warn(dead_code)]
fn read_json() -> Result<Config, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join(CONFIG_JSON_FILE);

    let json_str = fs::read_to_string(file_path)?;
    let data: Config = serde_json::from_str(&json_str)?;
    Ok(data)
}

/// The function `write_json` writes a given `Config` struct to a JSON file in a pretty-printed format.
///
/// Arguments:
///
/// * `config_data`: The `config_data` parameter is a reference to a `Config` struct.
///
/// Returns:
///
/// The function `write_json` is returning a `Result` enum with the success type `()` (unit) and the
/// error type `Box<dyn std::error::Error>`.
#[warn(dead_code)]
fn write_json(config_data: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join(CONFIG_JSON_FILE);

    let json_str = serde_json::to_string_pretty(config_data)?;
    fs::write(file_path, json_str)?;
    Ok(())
}

/// The `modify` function in Rust reads a JSON file, modifies a specific key-value pair, and writes the
/// updated data back to the file.
///
/// Arguments:
///
/// * `key`: The `key` parameter is a reference to a string (`&str`) which is used to identify a
/// specific field in a JSON data structure.
/// * `value`: The `value` parameter in the `modify` function is a reference to a string (`&str`). It is
/// the new value that will be assigned to a specific key in the JSON data structure after reading and
/// modifying it.
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



/// The function `get` retrieves a specific value from a JSON data structure based on the provided key.
/// 
/// Arguments:
/// 
/// * `key`: The `key` parameter in the `get` function is a reference to a string slice (`&str`) that
/// represents the key for which you want to retrieve a value from a JSON data structure.
/// 
/// Returns:
/// 
/// The `get` function returns a `Result<String, String>`. If the key is found in the JSON data, it
/// returns the corresponding value as a `String` wrapped in `Ok`. If the key is not found, it returns
/// an error message as a `String` wrapped in `Err`. If there is an error reading the JSON data, it also
/// returns an error message as a String wrapped in `Err`.
#[warn(dead_code)]
pub fn get(key: &str) -> Result<String, String> {
    match read_json() {
        Ok(data) => match key {
            "host_id" => Ok(data.host_id),
            _ => Err(String::from("Key not found")),
        },
        Err(_) => Err(String::from("Failed to read JSON")),
    }
}
