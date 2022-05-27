use std::fs::{read, create_dir_all};
use std::path::PathBuf;
use std::env::var_os;
use yaml_rust::{YamlLoader, Yaml};

use crate::errors::AppError;

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
        let content = default_config_file()?;
        
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
    let home_var = match var_os("HOME") {
        Some(value) => value,
        None => return Err(AppError::new("Failed to read HOME variable")),
    };

    let home_path = PathBuf::from(home_var);

    Ok(home_path)
}

fn user_name() -> Result<String, AppError> {
    let user_var = match var_os("USERNAME") {
        Some(value) => value,
        None => return Err(AppError::new("Failed to read USERNAME variable")),
    };

    match user_var.into_string() {
        Ok(value) => Ok(value),
        Err(_) => Err(AppError::new("Failed to read USERNAME variable")),
    }
}

fn default_config_file() -> Result<String, AppError> {
    let user_name = user_name()?;

    let content = format!(r#"
sections:
  - items:
    - name: Open settings
      command: gedit
      args: /home/{}/.config/quick-commands/config.yaml"#, user_name);

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::user_name;

    #[test]
    fn should_be_able_to_read_user_name() {
        if let Ok(user) = user_name() {
            assert_ne!("", user);
            return;
        }

        panic!("oh no!");
    }

}