use anyhow::Ok;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u32,
    username: String,
    password: String,
}

fn read_config() -> Result<DatabaseConfig, anyhow::Error> {
    let config: DatabaseConfig = confy::load("my_app_name", None)?;

    Ok(config)
}

fn save_config(config: &DatabaseConfig) -> Result<(), anyhow::Error> {
    confy::store("my_app_name", None, config)?;

    Ok(())
}

fn main() {
    match read_config() {
        config => {
            println!("{:?}", config);

            if let Err(err) = save_config(&config.unwrap()) {
                eprintln!("Failed to save configuration: {}", err);
            }
        }

        Err(err) => eprintln!("Failed to load configuration: {}", err),
    }
}
