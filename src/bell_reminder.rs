use std::thread::{spawn, sleep};
use std::time::Duration;
use std::process::Command;

use super::Configuration;

pub fn setup_bell_reminder(configuration: &Configuration) {
    if let Some(timeout) = configuration.bell_reminder {
        spawn(move || {
            loop {
                let duration = Duration::from_secs(timeout);
                sleep(duration);

                play_bell_reminder();
            }
        });
    }
}

fn play_bell_reminder() {
    let _ = Command::new("pw-play")
        .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
        .output();
}