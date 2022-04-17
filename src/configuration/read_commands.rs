// use crate::commands::Command;
// use crate::errors::AppError;

// use yaml_rust::Yaml;

// pub fn read_commands_from_yaml(yaml: &Yaml) -> Result<Vec<Command>, AppError> {
//     let mut commands = vec![];

//     for cmd in yaml["commands"].as_vec().unwrap() {
//         let mut name = String::from("");
//         if let yaml_rust::Yaml::String(ref value) = &cmd["name"] {
//             name = value.to_owned();
//         }

//         let mut command = String::from("");
//         if let yaml_rust::Yaml::String(ref value) = &cmd["command"] {
//             command = value.to_owned();
//         }

//         let mut args = String::from("");
//         if let yaml_rust::Yaml::String(ref value) = &cmd["args"] {
//             args = value.to_owned();
//         }

//         commands.push(Command::new(name, command, args));
//     }

//     Ok(commands)
// }