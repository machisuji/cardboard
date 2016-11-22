extern crate yaml_rust;

use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;

use self::yaml_rust::YamlLoader;
use self::yaml_rust::Yaml;

pub fn read_map_from_string_to_string(doc: &yaml_rust::Yaml) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    match doc {
        &Yaml::Hash(ref hash) => {
            for (yaml_key, yaml_value) in hash {
                if let &Yaml::String(ref key) = yaml_key {
                    if let &Yaml::String(ref value) = yaml_value {
                        result.insert(String::from(key.as_str()), String::from(value.as_str()));
                    }
                }
            }
        },
        _ => {

        }
    }

    result
}

fn read_yaml_object(file_name: String, key: &str) -> Option<HashMap<String, String>> {
    if let Some(mut file) = File::open(file_name).ok() {
        let mut yaml = String::new();

        if file.read_to_string(&mut yaml).is_ok() {
            if let Some(documents) = YamlLoader::load_from_str(& yaml).ok() {
                let doc: & Yaml = & documents[0];

                match doc {
                    &yaml_rust::Yaml::Hash(ref object) => {
                        for (yaml_key, yaml_value) in object {
                            if let &yaml_rust::Yaml::String(ref k) = yaml_key {
                                if k == key {
                                    return Some(read_map_from_string_to_string(yaml_value));
                                }
                            }
                        }
                    },
                    _ => { }
                }
            }
        }
    }

    None
}
