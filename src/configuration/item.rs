use yaml_rust::Yaml;

pub struct Item {
    pub name: String,
    pub command: String,
    pub args: String,
}

pub fn read_items_from_yaml(yaml: &Yaml) -> Vec<Item> {
    let mut items = Vec::new();

    if let Some(yaml) = yaml["items"].as_vec() {
        for yaml in yaml {
            let name = match &yaml["name"] {
                yaml_rust::Yaml::String(ref value) => value.to_owned(),
                _ => continue,
            };
    
            let command = match &yaml["command"] {
                yaml_rust::Yaml::String(ref value) => value.to_owned(),
                _ => continue,
            };
    
            let args = match &yaml["args"] {
                yaml_rust::Yaml::String(ref value) => value.to_owned(),
                _ => String::from(""),
            };
    
            items.push(Item { 
                name,
                command,
                args,
            });
        }
    }

    items
}