use clap::{Command, Arg};
use serde_yaml::Value;
use std::{fs, ops::Index};
use chrono::prelude::*;
use std::path::Path;

fn main() {
    let matches = Command::new("yw")
        .version("1.0")
        .author("Marcio Parente <support@deixei.com>")
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
        .subcommand(
            Command::new("execute")
                .about("Executes commands found in YAML")
                .arg(
                    Arg::new("input1")
                        .short('a')
                        .long("input1")
                        .value_name("FILE")
                        .help("Sets the input 1 file, for execution")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file, for execution results")
                        .required(true),
                ),
        )        
        .get_matches();
    
    //let config = read_config_file();
    //println!("Configuration loaded var1: {:?}", config["var1"].as_str().unwrap());
    
    //let environment_variables = load_environment_variables();
    //println!("Environment variables ALLUSERSPROFILE: {:?}", environment_variables["ALLUSERSPROFILE"].as_str().unwrap());

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
        //set_nested_value(&mut merged_yaml, "version", Value::String("marcio".to_string()));

        // Access a nested value using a path
        if let Some(nested_value) = get_nested_value(&merged_yaml, "version") {
            println!("version: {:?}", nested_value);
        } else {
            println!("version not found");
        }        

        let output_yaml = serde_yaml::to_string(&merged_yaml).unwrap();

        let mut output_yaml_string = output_yaml.to_string();
        while output_yaml_string.contains("{{") {
            output_yaml_string = replace_placeholders(&output_yaml_string, &merged_yaml);
        }

        if output_yaml_string.contains("{{") {
            eprintln!("Output path contains unresolved variables: {}", output_yaml_string);
            std::process::exit(1);
        }

        fs::write(output_path, output_yaml_string).unwrap();
    }


    if let Some(matches) = matches.subcommand_matches("execute") {
        let input_path: &String = matches.get_one::<String>("input1").unwrap();
        let output_path = matches.get_one::<String>("output").unwrap();

        let mut yaml = Value::Null;
        let mut output_yaml = Value::Null;

        let path_in = Path::new(input_path);
        let path_out = Path::new(output_path);

        if path_in.exists() == false {
            eprintln!("File does not exist: {}", input_path);
            std::process::exit(1);
        }

        merge_yaml_file(path_in, &mut yaml);

        //set_nested_value(&mut output_yaml, "execution.date", Value::String("{{get_date()}}".to_string()));

        let commands = get_nested_value(&yaml, "commands").unwrap();
        if commands.is_null() {
            eprintln!("Commands not found in the input file");
            std::process::exit(1);
        }
        let commands = commands.as_sequence().unwrap();
        let mut command_index = 0;
        for command in commands.iter() {
            command_index += 1;
            let default_name: String = format!("Command Index {}", command_index);
            let default_output: String = format!("output_{}", command_index);
            let task = command.as_mapping().unwrap();

            // in case task key is eq to "task" print a task, in case of "loop" print a loop command
            if let Some(task) = task.get(&Value::String("task".to_string())) {
                //println!("Task: {:?}", task);
                // a task must have a cmd, and optionally a name, description, output, and a execute flag
                
                let cmd = task.get(&Value::String("cmd".to_string())).unwrap().as_str().unwrap();
                let name = task.get(&Value::String("name".to_string())).unwrap_or(&Value::Null).as_str().unwrap_or(&default_name);
                let description = task.get(&Value::String("description".to_string())).unwrap_or(&Value::Null).as_str().unwrap_or("None");
                let output = task.get(&Value::String("output".to_string())).unwrap_or(&Value::Null).as_str().unwrap_or(&default_output);
                let execute = task.get(&Value::String("execute".to_string())).unwrap_or(&Value::Null).as_bool().unwrap_or(true);

                if execute {
                    //println!("Executing: {}", cmd);
                    let display_msg = format!("Executing: {}: {}", name, description);
                    println!("{}", display_msg);
                    let output_msg = format!("Output: {}", output);
                    println!("{}", output_msg);

                    // execute the command
                    let execute_command_output = std::process::Command::new("cmd").arg("/c").arg(cmd).output().unwrap();
                    let execute_command_output_value = Value::String(String::from_utf8_lossy(&execute_command_output.stdout).to_string());
                    
                    //println!("Command Result: {}", execute_command_output_str);
                    // create a Value mapping with the output of the command, and the key as the output index
                    //let mut v = Value::Mapping(serde_yaml::Mapping::new());
                    //v.as_mapping_mut().unwrap().insert(Value::String(output.to_string()), Value::String(execute_command_output_str.to_string()));

                    println!("Output: {:?}", execute_command_output_value);

                    set_nested_value(&mut output_yaml, output, execute_command_output_value);


                } else {
                    println!("SKIP: Not executing: {}", cmd);
                }

            } else if let Some(loop_command) = task.get(&Value::String("loop".to_string())) {
                println!("Loop: {:?}", loop_command);
            } else if let Some(conditional_command) = task.get(&Value::String("cmd.sh".to_string())) {
                println!("cmd.sh: {:?}", conditional_command);
            } else {
                eprintln!("Command task not known in the input file");
                //std::process::exit(1);
            }
            
            //println!("Command: {:?}", task);
        }

        let output_yaml_string = serde_yaml::to_string(&output_yaml).unwrap();
        save_to_file(path_out, &output_yaml_string);
    }

}


fn replace_placeholders(output_yaml: &str, merged_yaml: &Value) -> String {
    let re = regex::Regex::new(r"\{\{([^{}]*)\}\}").unwrap();
    re.replace_all(output_yaml, |caps: &regex::Captures| {
        let key = caps.get(1).unwrap().as_str().trim();
        //DEBUG: println!("Key: {:?}", key);

        if key.contains("|") {
            handle_pipe(key, merged_yaml)
        } else if key.contains("(") && key.contains(")") {
            apply_function(key, merged_yaml)
        } else if let Some(nested_value) = get_nested_value(merged_yaml, key) {
            //println!("Nested value: {:?}", nested_value);
            nested_value.as_str().unwrap().to_string()
        } else {
            //println!("Nested value not found");
            "".to_string()
        }
    }).to_string()
}

fn get_nested_value<'a>(yaml_value: &'a Value, path: &str) -> Option<&'a Value> {
    //DEBUG: println!("yaml_value: {:?}", yaml_value);
    path.split('.')
        .map(str::trim)
        .try_fold(yaml_value, |current_value, key| current_value.get(key))
}

fn set_nested_value(value: &mut Value, path: &str, new_value: Value) {
    // this function is to set a value in a nested path, updating the value if exists, or creating it if it does not exist
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

            // if the key does not exist, create it
            if let Value::Null = current_value {
                let mut map = Value::Mapping(serde_yaml::Mapping::new());
                map.as_mapping_mut().unwrap().insert(Value::String(key.to_string()), new_value.clone());
            }
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


// TODO Features
// - read a configuration file, as yaml.
// - read secrets from a file that is secure in the filesystem
// - load the configurations as a static structure
// - read environment variables

fn read_config_file() -> Value {
    // check if the file exists
    let path = Path::new("config.yaml");
    if path.exists() == false {
        eprintln!("File does not exist: {}", path.display());
        std::process::exit(1);
    }
    let config_file = fs::read_to_string("config.yaml").unwrap();
    let config: Value = serde_yaml::from_str(&config_file).unwrap();
    //DEBUG: println!("Config file loaded with: {:?}", config);
    return config;
}

fn load_environment_variables() -> Value {
    let env_vars_origin: Vec<(String, String)> = std::env::vars().collect();
    // convert to a yaml Value, mapping key and value
   
    let mut env_vars = Value::Mapping(serde_yaml::Mapping::new());
    for (key, value) in env_vars_origin {
        env_vars.as_mapping_mut().unwrap().insert(Value::String(key), Value::String(value));
    }
    //DEBUG: println!("env_vars: {:?}", env_vars);

    return env_vars;
}

fn apply_function(function_statement: &str, merged_yaml: &Value) -> String {
    // a function_statement is a string that contains a function with parameters
    // it can be a function with paramenters (e.g. get_env('ALLUSERSPROFILE') or get_data('2021-01-01', '2021-01-31'))
    // or a function without parameters (e.g. get_date())
    // or a function with a value key (e.g. get_value(root.level1.name))

    if function_statement == "" {
        return "".to_string();
    }

    if function_statement.contains("(") && function_statement.contains(")") {
        // it is a function
        let parts: Vec<&str> = function_statement.split("(").collect();
        let function_name = parts[0].trim();
        let params: Vec<&str> = parts[1].split(")").collect();
        let params: Vec<&str> = params[0].split(",").collect();
        let params: Vec<&str> = params.iter().map(|x| x.trim()).collect();

        //DEBUG: println!("apply_function :: function_statement: {}", function_statement);
        
        // Create a new vector to store the modified parameters
        // in reality this needs to be a vector of Value, but for now it is a vector of String
        let mut modified_params: Vec<String> = Vec::new();

        // params without quotes, like get_value(root.level1.name, 'demo_value'), need to get the value of the key
        // if the key is not found, return the default value
        if params != [""] {
            for param in params.iter() {
                if !param.contains("'") {
                    //DEBUG: 
                    //DEBUG: println!("param: {}", param);

                    let key = param.trim_matches('\'');
                    let value = get_nested_value(&merged_yaml, key).unwrap();
                    if value == &Value::Null {
                        // if the value is not found, return the default value
                        modified_params.push("default".to_string());
                    } else {
                        modified_params.push(value.as_str().unwrap().to_string());
                    }
                }
                else {
                    modified_params.push(param.trim_matches('\'').to_string());
                }
            }


            // functions with parameters

            // function with 1 parameter can be used as filters in a pipe
            if function_name == "upper" {
                let func_param_1 = modified_params.index(0);
                return func_param_1.to_uppercase();
            }
        
            if function_name == "lower" {
                let func_param_1 = modified_params.index(0);
                return func_param_1.to_lowercase();
            }
        
            if function_name == "len" {
                let func_param_1 = modified_params.index(0);
                return func_param_1.len().to_string();
            }

            if function_name == "is_empty" {
                let func_param_1 = modified_params.index(0);
                return func_param_1.is_empty().to_string();
            }

            if function_name == "is_not_empty" {
                let func_param_1 = modified_params.index(0);
                return (!func_param_1.is_empty()).to_string();
            }

            if function_name == "get_env" {
                // get the environment variable, if it does not exist, return an empty string
                // log an error if the environment variable does not exist
                let func_param_1 = modified_params.index(0);

                let environment_variables = load_environment_variables();
                let env_var: String = environment_variables.get(func_param_1).unwrap_or(&Value::String("".to_string())).as_str().unwrap().to_string(); 
                if env_var == "" {
                    eprintln!("Environment variable not found or empty: {}", func_param_1);
                }
                return env_var;
            }

            if function_name == "get_config" {
                let func_param_1 = modified_params.index(0);

                let config_variables = read_config_file();
                let config_var: String = config_variables.get(func_param_1).unwrap_or(&Value::String("".to_string())).as_str().unwrap().to_string(); 
                if config_var == "" {
                    eprintln!("Environment variable not found or empty: {}", func_param_1);
                }
                return config_var;
            }

            if function_name == "lookup_config" {
                // lookup_config('azure.prefix', 'resource_group')
                let func_param_1 = modified_params.index(0);
                let func_param_2 = modified_params.index(1);

                let config_variables = read_config_file();
                //println!("config_variables: {:?}", config_variables);
                let config_data = get_nested_value(&config_variables, func_param_1).unwrap();
                //println!("config_data: {:?}", config_data);
                
                // config_data is a Value, so we need to convert it to a dictionary
                let config_data = config_data.as_sequence().unwrap();
                // find in the dictionary the key func_param_1 as "id" and return "text"
                
                let result = config_data.iter()
                    .find(|x| x.get("id").and_then(|id| id.as_str()) == Some(func_param_2))
                    .and_then(|x| x.get("text").and_then(|text| text.as_str()))
                    .unwrap_or_else(|| {
                        eprintln!("Error: func_param_2 '{}' not found", func_param_2);
                        ""
                    });

                
                return result.to_string();

            }
    
            // if the function is get_data, get the date, this is an example of a function with parameters
            if function_name == "get_data" {
                let start_date = modified_params.index(0);
                let end_date = modified_params.index(1);
                return format!("{} - {}", start_date, end_date);
            }

            if function_name == "concat" {
                let start_str = modified_params.index(0);
                let end_str = modified_params.index(1);
                return format!("{}{}", start_str, end_str);
            }


        }

        // functions without parameters

        // if the function is get_date, get the date
        if function_name == "get_date" {
            return Utc::now().to_rfc3339();
        }

    }

    return "".to_string();
}

fn handle_pipe(pipe_statement: &str, merged_yaml: &Value) -> String {
    // at the moment the output of a pipe is a string, but it should be a Value
    let mut parts: Vec<&str> = pipe_statement.split('|').collect();
    if parts.len() < 2 {
        let result = apply_function(pipe_statement, merged_yaml);
        //println!("END PIPE: {}", result);
        return result;
    }

    let value = parts[0].trim();
    let value = if value.starts_with('\'') && value.ends_with('\'') {
        value.trim_matches('\'')
    } else {
        match get_nested_value(&merged_yaml, value) {
            Some(yaml_value) => yaml_value.as_str().unwrap_or(value),
            None => {
                eprintln!("Value not found: {}", value);
                value
            }
        }
    };

    let action = parts[1].trim();
    parts.drain(0..2);

    // this is an assumption that the input params are always strings, but is should be a Value.
    let function_statement = format!("{}('{}')", action, value);
    let result = apply_function(&function_statement, merged_yaml);

    if parts.is_empty() {
        //println!("NO PIPE: {}", result);
        return result;
    }

    let next_pipe = parts.join("|");
    let new_pipe_statement = format!("{}|{}", result, next_pipe);

    //println!("NEW PIPE: {}", new_pipe_statement);

    handle_pipe(&new_pipe_statement, merged_yaml)
}



fn save_to_file(output_path: &Path, output_yaml: &String) {
    fs::write(output_path, output_yaml).unwrap();
}