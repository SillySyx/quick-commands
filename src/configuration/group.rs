use yaml_rust::Yaml;

use super::item::{Item, read_items_from_yaml};

pub struct Group {
    pub name: String,
    pub items: Vec<Item>,
}

pub fn read_groups_from_yaml(yaml: &Yaml) -> Vec<Group> {
    let mut groups = Vec::new();

    if yaml.is_badvalue() {
        return groups;
    }

    for yaml in yaml.as_vec().unwrap() {
        let mut name = String::from("");
        if let yaml_rust::Yaml::String(ref value) = &yaml["name"] {
            name = value.to_owned();
        }

        let items = read_items_from_yaml(&yaml["items"]);

        groups.push(Group { 
            name, 
            items,
        })
    }

    groups
}