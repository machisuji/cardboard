extern crate yaml_rust;

use std::io::prelude::*;
use std::fs::File;

use linked_hash_map::LinkedHashMap;

use self::yaml_rust::YamlLoader;
use self::yaml_rust::Yaml;

use yaml_utils;

#[derive(Debug, Clone)]
pub struct Config {
    pub boards: LinkedHashMap<String, String>
}

lazy_static! {
    pub static ref CONFIG: Config = {
        get_config()
    };
}

pub fn config() -> Config {
    let ref config: Config = *CONFIG;

    config.clone()
}

pub fn is_meta_yaml(line: &str) -> bool {
    line.starts_with("meta:") || // skip meta yamls
        line.starts_with(" ") || // more meta yaml
        line == ""               // possible empty lines
}

pub fn read_meta_yaml(source: String) -> String {
    source
        .lines()
        .take_while(|line| line.starts_with("meta:") || line.starts_with(" "))
        .collect::<Vec<&str>>()
        .join("\n")
}

fn get_config() -> Config {
    let config: Option<Config> = load_config();

    config.unwrap_or_else(|| default_configuration())
}

fn load_config() -> Option<Config> {
    if let Some(mut file) = File::open(".cardboard/config.yml").ok() {
        let mut yaml = String::new();

        if file.read_to_string(&mut yaml).is_ok() {
            if let Some(documents) = YamlLoader::load_from_str(& yaml).ok() {
                let doc: & Yaml = & documents[0];
                let config: Config = read_config(doc);

                return Some(config);
            }
        }
    }

    None
}

fn default_configuration() -> Config {
    println!("Using default configuration");

    Config {
        boards: hash_from_string_to_string!{ "backlog" => "Backlog" }
    }
}

fn read_config(doc: & yaml_rust::Yaml) -> Config {
    let mut boards = LinkedHashMap::new();

    match doc {
        &yaml_rust::Yaml::Hash(ref config) => {
            for (yaml_key, yaml_value) in config {
                if let &yaml_rust::Yaml::String(ref key) = yaml_key {
                    if key == "boards" {
                        boards = yaml_utils::read_map_from_string_to_string(yaml_value);
                    }
                }
            }
        },
        _ => { }
    }

    Config {
        boards: boards
    }
}
