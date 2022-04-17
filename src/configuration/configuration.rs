use yaml_rust::Yaml;

use super::read_config::read_yaml_file;

use crate::errors::AppError;
use super::section::{Section, read_sections_from_yaml};

pub struct Configuration {
    pub text: Option<String>,
    pub icon: Option<String>,
    pub sections: Vec<Section>,
}

impl Configuration {
    pub fn load_yaml_file() -> Result<Self, AppError> {
        let yaml = read_yaml_file()?;
        let doc = &yaml[0];

        let text = read_text_from_yaml(&doc);
        let icon = read_icon_from_yaml(&doc);
        let sections = read_sections_from_yaml(&doc);

        Ok(Self {
            text,
            icon,
            sections,
        })
    }
}

fn read_text_from_yaml(yaml: &Yaml) -> Option<String> {
    match &yaml["text"] {
        yaml_rust::Yaml::String(ref value) => Some(value.to_owned()),
        _ => None,
    }
}

fn read_icon_from_yaml(yaml: &Yaml) -> Option<String> {
    match &yaml["icon"] {
        yaml_rust::Yaml::String(ref value) => Some(value.to_owned()),
        _ => None,
    }
}