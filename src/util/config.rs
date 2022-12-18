use rocket::serde::{Serialize, Deserialize};
use rand::{prelude::*, distributions::Alphanumeric};
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub secrets: Vec<String>,
    pub allowed_extensions: Vec<String>,
    pub upload_folder: String,
}

const CONFIG_PATH: &str = "config.toml";

pub fn load_config() -> Config {
    let config_file = std::fs::read_to_string(CONFIG_PATH);

    if config_file.is_err() {
        println!("Failed to read config file, creating new one");
        println!("Error: {}", config_file.err().unwrap());
        
        //default config
        let default_config: Config = Config {
            secrets: [create_secret()].to_vec(),
            allowed_extensions: [
                String::from("png"),
                String::from("jpg"),
                String::from("jpeg"),
                String::from("bmp"),
                String::from("gif"),
                String::from("txt"),
                String::from("mp4")
            ].to_vec(),
            upload_folder: String::from("uploads")
        };
        
        //save config
        let serialized = toml::to_string(&default_config).expect("Failed to serialize config file");
        std::fs::write(CONFIG_PATH, serialized).expect("Failed to write config file");

        return default_config;
    }

    return toml::from_str(&config_file.unwrap()).expect("Failed to parse config file");
}

pub fn create_secret() -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
}