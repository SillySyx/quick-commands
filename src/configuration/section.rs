use yaml_rust::Yaml;

use super::group::{Group, read_groups_from_yaml};
use super::item::{Item, read_items_from_yaml};

pub struct Section {
    pub groups: Vec<Group>,
    pub items: Vec<Item>,
}

pub fn read_sections_from_yaml(yaml: &Yaml) -> Vec<Section> {
    let mut sections = Vec::new();

    if yaml["sections"].is_badvalue() {
        return sections;
    }

    if let Some(yaml) = yaml["sections"].as_vec() {
        for section in yaml {
            let groups = read_groups_from_yaml(&section);
            let items = read_items_from_yaml(&section);
    
            sections.push(Section{
                groups,
                items,
            });
        }
    }

    sections
}
