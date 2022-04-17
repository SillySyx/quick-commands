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

        // let mut name = String::from("");
        // if let yaml_rust::Yaml::String(ref value) = &cmd["name"] {
        //     name = value.to_owned();
        // }

        // let mut command = String::from("");
        // if let yaml_rust::Yaml::String(ref value) = &cmd["command"] {
        //     command = value.to_owned();
        // }

        // let mut args = String::from("");
        // if let yaml_rust::Yaml::String(ref value) = &cmd["args"] {
        //     args = value.to_owned();
        // }

        // commands.push(Command::new(name, command, args));
    }

    sections

    // vec![
    //     Section {
    //         groups: vec![Group {
    //             name: String::from("Stuff"),
    //             items: vec![Item {
    //                 name: String::from("Text editor"),
    //                 command: String::from("gnome-text-editor"),
    //                 args: String::from(""),
    //             }],
    //         }],
    //         items: vec![Item {
    //             name: String::from("Text editor 2"),
    //             command: String::from("gnome-text-editor"),
    //             args: String::from(""),
    //         }],
    //     },
    //     Section {
    //         groups: vec![],
    //         items: vec![Item {
    //             name: String::from("Text editor 3"),
    //             command: String::from("gnome-text-editor"),
    //             args: String::from(""),
    //         }],
    //     },
    // ]
}
