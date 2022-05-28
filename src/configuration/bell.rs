use yaml_rust::Yaml;

pub struct Bell {
    pub interval: u64,
    pub volume: f64,
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

    Some(Bell {
        interval,
        volume,
    })
}