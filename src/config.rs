use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

/// Struct that stores the configuration for the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The interval in milliseconds between each update.
    ///
    /// This is the time between each call to the `update` function.
    pub update_interval: u64,
}

impl Config {
    /// Load the config from disk
    ///
    /// This function will load the config from disk and return it.
    /// If the config file does not exist, it will create a new one
    /// with the default values.
    pub fn load() -> Self {
        let project_dirs = ProjectDirs::from("io.github", "DitherWither", "icy-sysmonitor")
            .expect("Could not get project directories");

        let config_dir = project_dirs.config_dir();

        let config_path = config_dir.join("config.toml");

        // Load the config from disk if it exists
        // TODO: Add error handling
        if config_path.exists() {
            let config = std::fs::read_to_string(config_path).expect("Could not read config file");

            toml::from_str(&config).expect("Could not parse config file")
        } else {
            // Create the config directory if it does not exist
            if !config_dir.exists() {
                std::fs::create_dir_all(config_dir).expect("Could not create config directory");
            }

            // Create a new config with the default values
            let config = Self::default();

            // Write the config to disk
            let config_str = toml::to_string(&config).expect("Could not serialize config");

            std::fs::write(config_path, config_str).expect("Could not write config file");

            config
        }
    }

    /// Save the config to disk
    ///
    /// This function will save the config to disk.
    /// If the config file does not exist, it will create a new one.
    /// If the config file does exist, it will overwrite it.
    pub fn save(&self) {
        let project_dirs = ProjectDirs::from("io.github", "DitherWither", "icy-sysmonitor")
            .expect("Could not get project directories");

        let config_dir = project_dirs.config_dir();

        let config_path = config_dir.join("config.toml");

        // Create the config directory if it does not exist
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).expect("Could not create config directory");
        }

        // Write the config to disk
        let config_str = toml::to_string(&self).expect("Could not serialize config");

        std::fs::write(config_path, config_str).expect("Could not write config file");
    }
}

impl Default for Config {
    /// Create a new config with the default values.
    fn default() -> Self {
        Self {
            update_interval: 1000,
        }
    }
}
