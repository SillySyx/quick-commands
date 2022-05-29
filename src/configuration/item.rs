use yaml_rust::Yaml;
use crate::home_folder::resolve_home_folder;

pub struct Item {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
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
                Some(value) => resolve_home_folder(value.to_owned()),
                _ => continue,
            };
    
            let args = match yaml["args"].as_str() {
                Some(value) => parse_args(value),
                _ => vec![],
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

fn parse_args(args: &str) -> Vec<String> {
    args.split(" ")
        .map(|arg| resolve_home_folder(arg))
        .collect::<Vec<String>>()
}