use std::thread::{spawn, sleep};
use std::time::Duration;
use std::process::Command;

use super::Configuration;

pub fn setup_bell(configuration: &Configuration) {
    if let Some(bell) = &configuration.bell {
        spawn({
            let interval = bell.interval.clone();
            let volume = bell.volume.clone();
            move || {
                loop {
                    let duration = Duration::from_secs(interval);
                    sleep(duration);

                    play_bell_reminder(volume);
                }
            }
        });
    }
}

fn play_bell_reminder(volume: f64) {
    let _ = Command::new("pw-play")
        .arg("--volume")
        .arg(format!("{}", volume))
        .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
        .output();
}