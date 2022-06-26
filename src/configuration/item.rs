use std::collections::HashMap;

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
    let (args, parameters) = replace_quoted_values_with_parameters(args);

    args.split(" ")
        .map(|arg| {
            let arg = replace_parameter_with_quoted_value(arg, &parameters);
            resolve_home_folder(arg)
        })
        .collect::<Vec<String>>()
}

fn replace_quoted_values_with_parameters(value: &str) -> (String, HashMap<usize, String>) {
    let quoted_values = find_quoted_values_within_string(value);

    let mut new_value = value.to_string();
    let mut parameters = HashMap::new();

    for (start, end) in quoted_values {
        let arg = value[start..end].to_string();
        let index = parameters.len();

        parameters.insert(index, arg.clone());

        new_value = new_value.replace(&format!("\"{}\"", arg), &format!("$ARG_{}", index));
    }

    (new_value, parameters)
}

fn replace_parameter_with_quoted_value(value: &str, parameters: &HashMap<usize, String>) -> String {
    if !value.starts_with("$ARG_") {
        return value.to_string();
    }

    match value[5..].parse::<usize>() {
        Ok(index) => {
            match parameters.get(&index) {
                Some(value) => value.to_owned(),
                None => value.to_string(),
            }
        },
        Err(_) => value.to_string()
    }
}

fn find_quoted_values_within_string(args: &str) -> Vec<(usize, usize)> {
    let mut quotes = vec![];
    let mut positions = vec![];

    for (position, char) in args.split("").enumerate() {
        if char == "\"" {
            positions.push(position);
        }
    }

    let mut iter = positions.iter();
    loop {
        let start = match iter.next() {
            Some(value) => value,
            None => break,
        };
        let end = match iter.next() {
            Some(value) => value,
            None => break,
        };

        quotes.push((start.clone(), end.clone() - 1));
    }

    quotes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_parse_args_with_double_quotes() {
        let args = "-c \"$(pidof test)\"";
        let result = parse_args(args);

        let expected_result = vec!["-c", "$(pidof test)"];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_be_able_to_parse_args_with_multiple_double_quotes() {
        let args = "-c \"hello world\" \"$(pidof test)\"";
        let result = parse_args(args);

        let expected_result = vec!["-c", "hello world", "$(pidof test)"];
        assert_eq!(result, expected_result);
    }
}