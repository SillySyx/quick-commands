use chrono::{Local, DateTime};
use std::thread::{spawn, sleep};
use std::time::Duration;
use std::process::Command;

use super::{Configuration, configuration::Notification};

pub fn setup_notifications(configuration: &Configuration) {
    if configuration.notifications.len() == 0 {
        return;
    }

    let notifications = configuration.notifications.clone();
    let mut last_time = Local::now();

    spawn(move || {
        loop {
            if let Some(notifications) = find_notifications_to_show(&notifications, &last_time) {
                for notification in notifications {
                    display_notification(&notification);
                }
            }

            last_time = Local::now();
            
            if let Some(duration) = find_duration_to_sleep_until_next_notification(&notifications, &last_time) {
                sleep(duration);
                continue;
            }

            let duration = duration_until_midnight();
            sleep(duration);
        }
    });
}

fn find_notifications_to_show(notifications: &Vec<Notification>, last_time: &DateTime<Local>) -> Option<Vec<Notification>> {
    let now = Local::now();

    let notifications: Vec<Notification> = notifications
        .iter()
        .filter({
            let last_time = last_time.time();
            let now = now.time();
            move |notification| {
                notification.time > last_time && notification.time < now
            }
        })
        .map(|notification| notification.clone())
        .collect();

    if notifications.len() > 0 {
        return Some(notifications);
    }

    None
}

fn display_notification(notification: &Notification) {
    let mut args = vec![
        format!("'{}'", notification.title), 
        format!("'{}'", notification.text), 
    ];

    if notification.urgent {
        args.push(format!("-u"));
        args.push(format!("critical"));
    }

    let _ = Command::new("notify-send")
        .args(args)
        .output();
}

fn find_duration_to_sleep_until_next_notification(notifications: &Vec<Notification>, last_time: &DateTime<Local>) -> Option<Duration> {
    let mut notifications: Vec<Notification> = notifications
        .iter()
        .filter_map({
            let last_time = last_time.time();
            move |notification| {
                if notification.time > last_time {
                    return Some(notification.clone());
                }

                None
            }
        })
        .collect();

    if notifications.len() == 0 {
        return None;
    }

    notifications.sort_by(|a, b| {
        a.time.partial_cmp(&b.time).unwrap()
    });

    if let Some(notification) = notifications.get(0) {
        let duration = notification.time - last_time.time();
        return match duration.to_std() {
            Ok(value) => Some(value),
            Err(_) => None,
        };
    }

    None
}

fn duration_until_midnight() -> Duration {
    let now = Local::now();
    let midnight = (now + chrono::Duration::days(1)).date().and_hms(0, 0, 0);
    let duration = midnight.signed_duration_since(now).to_std().unwrap();

    duration
}