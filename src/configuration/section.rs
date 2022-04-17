use yaml_rust::Yaml;

use super::group::{Group, read_groups_from_yaml};
use super::item::{Item, read_items_from_yaml};

pub struct Section {
    pub groups: Vec<Group>,
    pub items: Vec<Item>,
}

pub fn read_sections_from_yaml(yaml: &Yaml) -> Vec<Section> {
    let mut sections = Vec::new();

    for section in yaml["sections"].as_vec().unwrap() {
        let groups = read_groups_from_yaml(&section["groups"]);
        let items = read_items_from_yaml(&section["items"]);

        sections.push(Section{
            groups,
            items,
        });
    }

    sections
}
