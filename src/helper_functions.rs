pub mod helpers {

    use dirs::config_dir;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string, Value};
    use std::fs::read_to_string;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ConfigInterface {
        pub flight_club: Value,
        pub electric_shuffle: Value,
        pub red_engine: Value,
    }

    pub fn create_default_config_files() {
        let mut config_root: PathBuf = config_dir().unwrap();
        config_root.push("CC");

        std::fs::create_dir_all(&config_root).expect("Failed to create dirs");
        config_root.push("Config.txt");

        let mut config_file: File = File::create(&config_root).unwrap();
        println!("Config Root Path: {}", config_root.to_str().unwrap());

        let default_config: ConfigInterface = ConfigInterface {
            flight_club: Value::String(String::new()),
            electric_shuffle: Value::String(String::new()),
            red_engine: Value::String(String::new()),
        };

        let json_data: String = to_string(&default_config).unwrap();
        _ = config_file.write_all(json_data.as_bytes());

        println!(
            "Config Has been created. Set cache paths in: {:?}",
            config_root
        );

        println!("Once paths are set, try again.\n");
    }

    pub fn read_config() -> ConfigInterface {
        let mut config_path: PathBuf = config_dir().unwrap();
        config_path.push("CC");
        config_path.push("Config.txt");

        if !config_path.exists() {
            println!("No Config Found. Creating...");
            create_default_config_files();
        }

        let config_value: &str = &read_to_string(config_path).unwrap();

        let escaped_config_value = config_value.replace("\\", "\\\\");

        let user_config: ConfigInterface = from_str(&escaped_config_value).unwrap();

        return user_config;
    }
}
