use super::read_commands::read_commands_from_yaml;
use super::read_config::read_yaml_file;

use crate::commands::Command;
use crate::errors::AppError;

pub struct Configuration {
    pub commands: Vec<Command>,
}

impl Configuration {
    pub fn load_yaml_file() -> Result<Self, AppError> {
        let yaml = read_yaml_file()?;
        let commands = read_commands_from_yaml(&yaml[0])?;

        Ok(Self {
            commands,
        })
    }
}