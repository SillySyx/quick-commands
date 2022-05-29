use std::fs::{read, create_dir_all};
use std::path::PathBuf;
use yaml_rust::{YamlLoader, Yaml};

use crate::errors::AppError;
use crate::home_folder::read_home_from_environment_variables;

pub fn read_yaml_file() -> Result<Vec<Yaml>, AppError> {
    let content = read_config_file()?;
    let yaml = YamlLoader::load_from_str(&content).expect("msg");

    Ok(yaml)
}

fn read_config_file() -> Result<String, AppError> {
    let mut config_path = home_path()?;
    config_path.push(".config/quick-commands");

    if !config_path.exists() {
        match create_dir_all(&config_path) {
            Ok(_) => (),
            Err(_) => return Err(AppError::new("Failed to create config path")),
        };
    }

    config_path.push("config.yaml");
    
    if !config_path.exists() {
        let content = default_config_file();
        
        match std::fs::write(&config_path, content) {
            Ok(_) => (),
            Err(_) => return Err(AppError::new("Failed to save config file")),
        };
    }

    let file_bytes = match read(config_path) {
        Ok(bytes) => bytes,
        Err(_) => return Err(AppError::new("Failed to read config file")),
    };

    let file_content = match String::from_utf8(file_bytes) {
        Ok(value) => value,
        Err(_) => return Err(AppError::new("Failed to read config file")),
    };

    Ok(file_content)
}

fn home_path() -> Result<PathBuf, AppError> {
    let home = match read_home_from_environment_variables() {
        Some(value) => value,
        None => return Err(AppError::new("Failed to read HOME variable")),
    };

    let home_path = PathBuf::from(home);

    Ok(home_path)
}

fn default_config_file() -> &'static str {
    r#"
sections:
  - items:
    - name: Settings
      command: gnome-text-editor
      args: ~/.config/quick-commands/config.yaml"#
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_be_able_to_() {
    }
}