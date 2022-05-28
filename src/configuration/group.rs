use yaml_rust::Yaml;

use super::item::{Item, read_items_from_yaml};

pub struct Group {
    pub name: String,
    pub items: Vec<Item>,
}

pub fn read_groups_from_yaml(yaml: &Yaml) -> Vec<Group> {
    let mut groups = Vec::new();

    if let Some(yaml) = yaml["groups"].as_vec() {
        for yaml in yaml {
            let name = match yaml["name"].as_str() {
                Some(value) => value.to_owned(),
                None => continue,
            };

            let items = read_items_from_yaml(&yaml);
    
            groups.push(Group { 
                name, 
                items,
            });
        }
    }

    groups
}