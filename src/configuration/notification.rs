use yaml_rust::Yaml;
use chrono::{NaiveTime, Weekday};

#[derive(Clone)]
pub struct Notification {
    pub title: String,
    pub text: String,
    pub urgent: bool,
    pub time: NaiveTime,
    pub days: Vec<Weekday>,
}

impl Notification {
    pub fn is_weekday(&self, day: &Weekday) -> bool {
        self.days.contains(day)
    }
}

pub fn read_notifications_from_yaml(yaml: &Yaml) -> Vec<Notification> {
    let mut notifications = Vec::new();

    if yaml["notifications"].is_badvalue() {
        return notifications;
    }

    if let Some(yaml) = yaml["notifications"].as_vec() {
        for yaml in yaml {
            let title = match yaml["title"].as_str() {
                Some(value) => value.to_owned(),
                _ => continue,
            };

            let text = match yaml["text"].as_str() {
                Some(value) => value.to_owned(),
                _ => continue,
            };

            let time = match yaml["time"].as_str() {
                Some(value) => match parse_system_time(value) {
                    Some(value) => value,
                    None => continue,
                },
                _ => continue,
            };

            let urgent = match yaml["urgent"].as_bool() {
                Some(value) => value,
                _ => false,
            };

            let days = match yaml["days"].as_str() {
                Some(value) => parse_weekdays(value),
                _ => all_weekdays(),
            };
    
            notifications.push(Notification { 
                title,
                text,
                urgent,
                time,
                days,
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

fn parse_weekdays(value: &str) -> Vec<Weekday> {
    if value == "All" || value.is_empty() {
        return all_weekdays();
    }

    value
        .split(",")
        .map(|value| {
            match value {
                "Mon" => Weekday::Mon,
                "Tue" => Weekday::Tue,
                "Wed" => Weekday::Wed,
                "Thu" => Weekday::Thu,
                "Fri" => Weekday::Fri,
                "Sat" => Weekday::Sat,
                "Sun" => Weekday::Sun,
                _ => Weekday::Mon,
            }
        })
        .collect()
}

fn all_weekdays() -> Vec<Weekday> {
    vec![Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri, Weekday::Sat, Weekday::Sun]
}