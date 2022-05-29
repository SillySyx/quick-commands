use std::thread::{spawn, sleep};
use std::time::Duration;
use std::process::Command;

use super::Configuration;

pub fn setup_bell(configuration: &Configuration) {
    if let Some(bell) = &configuration.bell {
        spawn({
            let interval = bell.interval.clone();
            let volume = bell.volume.clone();
            let audio_file = bell.audio_file.clone();
            move || {
                loop {
                    let duration = Duration::from_secs(interval);
                    sleep(duration);

                    play_bell_reminder(volume, &audio_file);
                }
            }
        });
    }
}

fn play_bell_reminder(volume: f64, audio_file: &str) {
    let _ = Command::new("pw-play")
        .arg("--volume")
        .arg(format!("{}", volume))
        .arg(audio_file)
        .output();
}