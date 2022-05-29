use yaml_rust::Yaml;
use crate::home_folder::resolve_home_folder;

pub struct Bell {
    pub interval: u64,
    pub volume: f64,
    pub audio_file: String,
}

pub fn read_bell_from_yaml(yaml: &Yaml) -> Option<Bell> {
    let interval = match yaml["bell"]["interval"].as_i64() {
        Some(value) => value as u64,
        None => return None,
    };

    let volume = match yaml["bell"]["volume"].as_f64() {
        Some(value) => value.clamp(0.0, 1.0),
        None => 1.0,
    };

    let audio_file = match yaml["bell"]["file"].as_str() {
        Some(value) => resolve_home_folder(value.to_owned()),
        None => String::from("/usr/share/sounds/freedesktop/stereo/complete.oga"),
    };

    Some(Bell {
        interval,
        volume,
        audio_file,
    })
}