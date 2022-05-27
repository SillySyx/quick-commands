use yaml_rust::Yaml;

use super::read_config::read_yaml_file;

use crate::{errors::AppError, notifications::{Notification, read_notifications_from_yaml}};
use super::section::{Section, read_sections_from_yaml};

pub struct Configuration {
    pub text: Option<String>,
    pub icon: Option<String>,
    pub sections: Vec<Section>,
    pub notifications: Vec<Notification>,
    pub bell_reminder: Option<u64>,
}

impl Configuration {
    pub fn load_yaml_file() -> Result<Self, AppError> {
        let yaml = read_yaml_file()?;
        let doc = &yaml[0];

        let text = read_text_from_yaml(&doc);
        let icon = read_icon_from_yaml(&doc);
        let sections = read_sections_from_yaml(&doc);
        let notifications = read_notifications_from_yaml(&doc);
        let bell_reminder = read_bell_reminder_from_yaml(&doc);

        Ok(Self {
            text,
            icon,
            sections,
            notifications,
            bell_reminder,
        })
    }
}

fn read_text_from_yaml(yaml: &Yaml) -> Option<String> {
    if let Some(value) = yaml["text"].as_str() {
        return Some(value.to_owned());
    }

    None
}

fn read_icon_from_yaml(yaml: &Yaml) -> Option<String> {
    if let Some(value) = yaml["icon"].as_str() {
        return Some(value.to_owned());
    }

    None
}

fn read_bell_reminder_from_yaml(yaml: &Yaml) -> Option<u64> {
    if let Some(value) = yaml["bell reminder"].as_i64() {
        return Some(value as u64);
    }

    None
}