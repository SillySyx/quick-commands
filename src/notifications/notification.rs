use yaml_rust::Yaml;
use chrono::NaiveTime;

#[derive(Clone)]
pub struct Notification {
    pub title: String,
    pub text: String,
    pub urgent: bool,
    pub time: NaiveTime,
}

pub fn read_notifications_from_yaml(yaml: &Yaml) -> Vec<Notification> {
    let mut notifications = Vec::new();

    if yaml["notifications"].is_badvalue() {
        return notifications;
    }

    if let Some(yaml) = yaml["notifications"].as_vec() {
        for yaml in yaml {
            let title = match &yaml["title"] {
                yaml_rust::Yaml::String(ref value) => value.to_owned(),
                _ => continue,
            };
    
            let text = match &yaml["text"] {
                yaml_rust::Yaml::String(ref value) => value.to_owned(),
                _ => continue,
            };
    
            let time = match &yaml["time"] {
                yaml_rust::Yaml::String(ref value) => match parse_system_time(&value) {
                    Some(value) => value,
                    None => continue,
                },
                _ => continue,
            };
    
            let urgent = match &yaml["urgent"] {
                yaml_rust::Yaml::Boolean(ref value) => value.to_owned(),
                _ => false,
            };
    
            notifications.push(Notification { 
                title,
                text,
                urgent,
                time,
            });
        }
    }

    notifications
}

fn parse_system_time(value: &str) -> Option<NaiveTime> {
    match NaiveTime::parse_from_str(value, "%H:%M") {
        Ok(value) => Some(value),
        Err(_) => None
    }
}