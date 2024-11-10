use clap::{Command, Arg};
use serde_yaml::Value;
use std::fs;
use chrono::Utc;
use std::path::Path;

fn main() {
    let matches = Command::new("yw")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("Merges YAML files")
        .subcommand(
            Command::new("merge")
                .about("Merges YAML files")
                .arg(
                    Arg::new("input1")
                        .short('a')
                        .long("input1")
                        .value_name("FILE")
                        .help("Sets the input 1 file or directory")
                        .required(true),
                )
                .arg(
                    Arg::new("input2")
                        .short('b')
                        .long("input2")
                        .value_name("FILE")
                        .help("Sets the input 2 file or directory")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("merge") {
        let input1_paths: &String = matches.get_one::<String>("input1").unwrap();
        let input2_paths: &String = matches.get_one::<String>("input2").unwrap();
        let output_path = matches.get_one::<String>("output").unwrap();

        let mut merged_yaml = Value::Null;

        let mut input_paths: Vec<&str> = Vec::new();
        input_paths.push(input1_paths);
        input_paths.push(input2_paths);

        for input_path in input_paths {
            let path = Path::new(input_path);

            if path.exists() == false {
                eprintln!("File does not exist: {}", input_path);
                std::process::exit(1);
            }
       
            if path.is_dir() {
                eprint!("Directory not supported yet: {}", input_path);
            } else {
                merge_yaml_file(path, &mut merged_yaml);
            }
        }
        
        // Change the value of root.level1.name to "marcio"
        set_nested_value(&mut merged_yaml, "root.level1.name", Value::String("marcio".to_string()));

        // Access a nested value using a path
        if let Some(nested_value) = get_nested_value(&merged_yaml, "root.level1.name") {
            println!("Nested value: {:?}", nested_value);
        } else {
            println!("Nested value not found");
        }        

        let output_yaml = serde_yaml::to_string(&merged_yaml).unwrap();

        // find string inside {{ }} and replace it with the value of the nested key
        let re = regex::Regex::new(r"\{\{([^{}]*)\}\}").unwrap();
        let output_yaml = re.replace_all(&output_yaml, |caps: &regex::Captures| {
            let mut filter = "default";
            let mut key = caps.get(1).unwrap().as_str();
            // if the key contains a pipe, it means it has a filter
            if key.contains("|") {
                let parts: Vec<&str> = key.split("|").collect();
                key = parts[0].trim();
                filter = parts[1].trim();
                //print!("{} | {}", key, filter);
            }

            if key.contains("()") {
                let parts: Vec<&str> = key.split("()").collect();
                key = parts[0].trim();

                // if the key is a function, call it
                if key == "get_date" {
                    return chrono::Utc::now().to_rfc3339();
                }

                return "".to_string();
            }
            // trim the key
            key = key.trim();
            filter = filter.trim();

            println!("key:{}", key);
            println!("filter:{}", filter);
            let value = get_nested_value(&merged_yaml, key).unwrap();
            if value == &Value::Null {
                return "".to_string();
            } else {
                let final_value = value.as_str().unwrap();
                if filter == "default" {
                    return final_value.to_string();
                }
                else {
                    // apply the filter
                    // if filter == "upper", convert the value to uppercase
                    if filter == "upper" {
                        return final_value.to_uppercase();
                    }
                    if filter == "lower" {
                        return final_value.to_lowercase();
                    }
                    if filter == "len" {
                        return final_value.len().to_string();
                    }
                }
                return final_value.to_string();
            }
        });
        // Convert the result to a String
        let output_yaml = output_yaml.to_string();
        fs::write(output_path, output_yaml).unwrap();
    }
}

fn get_nested_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current_value = value;
    for key in path.split('.') {
        current_value = current_value.get(key)?;
    }
    Some(current_value)
}

fn set_nested_value(value: &mut Value, path: &str, new_value: Value) {
    let mut current_value = value;
    let keys: Vec<&str> = path.split('.').collect();
    for (i, key) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            if let Value::Mapping(map) = current_value {
                map.insert(Value::String(key.to_string()), new_value.clone());
            }
        } else {
            current_value = current_value
                .get_mut(*key)
                .unwrap_or_else(|| panic!("Key not found: {}", key));
        }
    }
}

fn merge_yaml_file(path: &Path, merged_yaml: &mut Value) {
    let file_content = fs::read_to_string(path).unwrap();
    // if file_content contains multiple documents (---), we need to split them and merge them separately
    if file_content.contains("---") {
        let documents: Vec<&str> = file_content.split("---").collect();
        for document in documents {
            let yaml: Value = serde_yaml::from_str(document).unwrap();
            merge_yaml(merged_yaml, &yaml);
        }
        return;
    } else {
        let yaml: Value = serde_yaml::from_str(&file_content).unwrap();
        merge_yaml(merged_yaml, &yaml);    
    }
}

fn merge_yaml(base: &mut Value, other: &Value) {
    match (base, other) {
        (Value::Mapping(base_map), Value::Mapping(other_map)) => {
            for (key, value) in other_map {
                merge_yaml(base_map.entry(key.clone()).or_insert(Value::Null), value);
            }
        }
        (Value::Sequence(base_seq), Value::Sequence(other_seq)) => {
            for value in other_seq {
                // if the value already exists in the base sequence, skip it
                if base_seq.contains(value) {
                    continue;
                }
                base_seq.push(value.clone());
            }
        }
        (base, other) => {
            *base = other.clone();
        }
    }
}