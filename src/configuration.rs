extern crate yaml_rust;

use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;

use self::yaml_rust::YamlLoader;
use self::yaml_rust::Yaml;

#[derive(Debug)]
pub struct Config {
    boards: Vec<String>
}

pub fn load_config() -> Option<Config> {
    if let Some(mut file) = File::open(".cardboard/config.yml").ok() {
        let mut yaml = String::new();

        if file.read_to_string(&mut yaml).is_ok() {
            if let Some(documents) = YamlLoader::load_from_str(& yaml).ok() {
                let doc = &documents[0];

                println!("Calling read_boards");
                let result = read_boards(doc);
                println!("result: {:?}", result);

                let config = Config {
                    boards: vec![String::from("foo"), String::from("bar"), String::from("baz")]
                };

                return Some(config)
            }
        }
    }

    None
}

fn read_boards(doc: &yaml_rust::Yaml) -> Vec<String> {
    match doc {
        &yaml_rust::Yaml::Hash(ref config) => {
            println!("config: {:?}", config);

            for (yaml_key, yaml_value) in config {
                if let &yaml_rust::Yaml::String(ref key) = yaml_key {
                    if key == "boards" {
                        println!("boards: {:?}", read_map_from_string_to_string(yaml_value));
                    }
                }
            }

            vec![]
        },
        _ => {
            println!("no match");
            vec![]
        }
    }
}

fn read_map_from_string_to_string(doc: &yaml_rust::Yaml) -> HashMap<&str, &str> {
    let mut result = HashMap::new();

    match doc {
        &Yaml::Hash(ref hash) => {
            for (yaml_key, yaml_value) in hash {
                if let &Yaml::String(ref key) = yaml_key {
                    if let &Yaml::String(ref value) = yaml_value {
                        result.insert(key.as_str(), value.as_str());
                    }
                }
            }
        },
        _ => {

        }
    }

    result
}
