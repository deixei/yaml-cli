use clap::{Command, Arg};
use serde_yaml::Value;
use std::fs;
use std::path::Path;
use std::collections::BTreeMap;

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

        let output_yaml = serde_yaml::to_string(&merged_yaml).unwrap();
        fs::write(output_path, output_yaml).unwrap();
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