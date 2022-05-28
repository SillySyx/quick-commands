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
            let name = match yaml["name"].as_str() {
                Some(value) => value.to_owned(),
                _ => continue,
            };
    
            let command = match yaml["command"].as_str() {
                Some(value) => value.to_owned(),
                _ => continue,
            };
    
            let args = match yaml["args"].as_str() {
                Some(value) => value.to_owned(),
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