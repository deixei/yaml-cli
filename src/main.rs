use chrono::prelude::*;
use clap::{Arg, Command};
use serde_yaml::Value;
use std::path::Path;
use std::{fs, ops::Index};

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => ({
        use colored::*;
        eprintln!("{} {}", "ERROR:".red(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => ({
        use colored::*;
        eprintln!("{} {}", "WARNING:".yellow(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{} {}", "INFO:".blue(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{} {}", "SUCCESS:".green(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_banner_yellow {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{}", format!($($arg)*).yellow());
    });
}

#[macro_export]
macro_rules! print_banner_green {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{}", format!($($arg)*).green());
    });
}

#[macro_export]
macro_rules! print_banner_red {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{}", format!($($arg)*).red());
    });
}

#[macro_export]
macro_rules! print_banner_blue {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{}", format!($($arg)*).blue());
    });
}

#[macro_export]
macro_rules! print_executing {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{} {}", "## Executing:".bold().green(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_output {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{} {}", "\t -> Output:".bold().blue(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_debug {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{} {}", "-> DEBUG:".bold().yellow(), format!($($arg)*));
    });
}

fn main() {
    let matches = Command::new("yw")
        .version("1.0")
        .author("Marcio Parente <support@deixei.com>")
        .about("Merges YAML files")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Sets the level of verbosity")
                .global(true)
                .required(false)
                .default_value("none")
                .value_parser(["none", "v", "vv", "vvv"]),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Sets the debug flag")
                .global(true)
                .action(clap::ArgAction::SetTrue),
        )
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
        run_subcommand_merge(matches);
    }

    if let Some(matches) = matches.subcommand_matches("execute") {
        run_subcommand_execute(matches);
    }
}

fn run_subcommand_merge(matches: &clap::ArgMatches) {
    let global_args = GlobalArguments::from_matches(matches);
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
            print_error!("File does not exist: {}", input_path);
            std::process::exit(1);
        }

        if path.is_dir() {
            print_error!("Directory not supported yet: {}", input_path);
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
        print_error!("version not found");
    }

    let output_yaml = serde_yaml::to_string(&merged_yaml).unwrap();

    let mut output_yaml_string = output_yaml.to_string();
    while output_yaml_string.contains("{{") {
        output_yaml_string = replace_placeholders(&output_yaml_string, &merged_yaml);
    }

    if output_yaml_string.contains("{{") {
        print_error!(
            "Output path contains unresolved variables: {}",
            output_yaml_string
        );
        std::process::exit(1);
    }

    fs::write(output_path, output_yaml_string).unwrap();
    global_args.display_summary();    
}

fn run_subcommand_execute(matches: &clap::ArgMatches) {
    let global_args = GlobalArguments::from_matches(matches);
    let input_path: &String = matches.get_one::<String>("input1").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    let mut yaml = Value::Null;
    let mut output_yaml = Value::Null;

    let path_in = Path::new(input_path);
    let path_out = Path::new(output_path);

    if path_in.exists() == false {
        print_error!("File does not exist: {}", input_path);
        std::process::exit(1);
    }

    merge_yaml_file(path_in, &mut yaml);

    //set_nested_value(&mut output_yaml, "execution.date", Value::String("{{get_date()}}".to_string()));

    let commands = get_nested_value(&yaml, "commands").unwrap();
    if commands.is_null() {
        print_error!("Commands not found in the input file");
        std::process::exit(1);
    }
    let commands = commands.as_sequence().unwrap();
    let mut command_index = 0;

    let mut counter = Counters {
        total: 0,
        executed: 0,
        skipped: 0,
        errors: 0,
    };

    for command in commands.iter() {
        run_command_task(&mut command_index, command, &mut counter, &mut output_yaml);
    }

    counter.display_summary();
    global_args.display_summary();

    let output_yaml_string = serde_yaml::to_string(&output_yaml).unwrap();
    save_to_file(path_out, &output_yaml_string);
}


struct GlobalArguments {
    verbose: String,
    debug: bool,
}

impl GlobalArguments {
    fn from_matches(matches: &clap::ArgMatches) -> Self {
        let global_debug_flag: bool = matches.get_flag("debug");
        //DEBUG: print_debug!("Global debug flag: {:?}", global_debug_flag);
        let global_verbose_level: String = matches.get_one::<String>("verbose").unwrap().clone();
        //DEBUG: print_debug!("Global verbose level: {:?}", global_verbose_level);
        return GlobalArguments { verbose: global_verbose_level, debug: global_debug_flag };
    }

    fn display_summary(&self) {
        if self.debug {
            println!("");
            print_banner_yellow!("### Global Arguments #####################################");
            print_warning!(
                "Debug: {}; Verbose: {};",
                self.debug,
                self.verbose
            );
            print_banner_yellow!("##########################################################");
        }
    }

}


struct Counters {
    total: i32,
    executed: i32,
    skipped: i32,
    errors: i32,
}

impl Counters {
    fn display_summary(&self) {
        println!("");
        print_banner_blue!("### Summary ##############################################");
        print_info!(
            "Total: {}; Executed: {}; Skipped: {}; Errors: {}",
            self.total,
            self.executed,
            self.skipped,
            self.errors
        );
        print_banner_blue!("##########################################################");
    }
}

#[derive(Debug)]
struct OSTask {
    name: String,
    description: String,
    execute: bool,
    debug_flag: bool,
    cmd: String,
    output: String,
}

impl OSTask {
    fn from_value(task: &Value, command_index: &i32) -> Self {
        let default_name: String = format!("Command Index {}", command_index);
        let default_output: String = format!("outs.output_{}", command_index);
        let os_task = OSTask {
            name: task
                .get(&Value::String("name".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or(&default_name)
                .to_string(),
            description: task
                .get(&Value::String("description".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or("None")
                .to_string(),
            execute: task
                .get(&Value::String("execute".to_string()))
                .unwrap_or(&Value::Null)
                .as_bool()
                .unwrap_or(true),
            debug_flag: task
                .get(&Value::String("debug".to_string()))
                .unwrap_or(&Value::Null)
                .as_bool()
                .unwrap_or(false),
            cmd: task
                .get(&Value::String("cmd".to_string()))
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            output: task
                .get(&Value::String("output".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or(&default_output)
                .to_string(),
        };
        return os_task;
    }
    fn display_message(&self) {
        let display_msg = format!("{}: {}", self.name, self.description);
        print_executing!("{}", display_msg);
    }
}

#[derive(Debug)]
struct ConsoleTask {
    name: String,
    description: String,
    execute: bool,
    debug_flag: bool,
    message: String,
    output: String,
}

impl ConsoleTask {
    fn from_value(task: &Value, command_index: &i32) -> Self {
        let default_name: String = format!("Command Index {}", command_index);
        let default_output: String = format!("outs.output_{}", command_index);
        let console_task = ConsoleTask {
            name: task
                .get(&Value::String("name".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or(&default_name)
                .to_string(),
            description: task
                .get(&Value::String("description".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or("None")
                .to_string(),
            execute: task
                .get(&Value::String("execute".to_string()))
                .unwrap_or(&Value::Null)
                .as_bool()
                .unwrap_or(true),
            debug_flag: task
                .get(&Value::String("debug".to_string()))
                .unwrap_or(&Value::Null)
                .as_bool()
                .unwrap_or(false),
            message: task
                .get(&Value::String("message".to_string()))
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            output: task
                .get(&Value::String("output".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or(&default_output)
                .to_string(),
        };
        return console_task;
    }

    fn display_message(&self) {
        let display_msg = format!("{}: {}", self.name, self.description);
        print_executing!("{}", display_msg);
    }
}

#[derive(Debug)]
struct HTTPTask {
    name: String,
    description: String,
    execute: bool,
    debug_flag: bool,
    url: String,
    output: String,
}

impl HTTPTask {
    fn from_value(task: &Value, command_index: &i32) -> Self {
        let default_name: String = format!("Command Index {}", command_index);
        let default_output: String = format!("outs.output_{}", command_index);
        let http_task = HTTPTask {
            name: task
                .get(&Value::String("name".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or(&default_name)
                .to_string(),
            description: task
                .get(&Value::String("description".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or("None")
                .to_string(),
            execute: task
                .get(&Value::String("execute".to_string()))
                .unwrap_or(&Value::Null)
                .as_bool()
                .unwrap_or(true),
            debug_flag: task
                .get(&Value::String("debug".to_string()))
                .unwrap_or(&Value::Null)
                .as_bool()
                .unwrap_or(false),
            url: task
                .get(&Value::String("url".to_string()))
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            output: task
                .get(&Value::String("output".to_string()))
                .unwrap_or(&Value::Null)
                .as_str()
                .unwrap_or(&default_output)
                .to_string(),
        };
        return http_task;
    }
    fn display_message(&self) {
        let display_msg = format!("{}: {}", self.name, self.description);
        print_executing!("{}", display_msg);
    }
}

fn run_command_task(
    command_index: &mut i32,
    command: &Value,
    counter: &mut Counters,
    output_yaml: &mut Value,
) {
    *command_index += 1;
    let task = command.as_mapping().unwrap();

    // some examples of commands to handle
    // os.win.cmd: cmd: 'dir'
    // os.linux.cmd: cmd: 'ls -la'
    // os.win.ps: cmd: 'Get-Process'
    // os.cmd: cmd: 'dir'
    // loop.for: start: 1 end: 10 cmd: 'echo {{get_date()}}'

    // in case task key is eq to "task" print a task, in case of "loop" print a loop command
    if let Some(task) = task.get(&Value::String("os.win.cmd".to_string())) {
        let os_task: OSTask = OSTask::from_value(task, command_index);
        run_task_os_win_cmd(&os_task, counter, output_yaml);
    } else if let Some(task) = task.get(&Value::String("os.linux.cmd".to_string())) {
        let os_task: OSTask = OSTask::from_value(task, command_index);
        run_task_os_linux_cmd(&os_task, counter, output_yaml);
    } else if let Some(task) = task.get(&Value::String("os.win.ps".to_string())) {
        let os_task: OSTask = OSTask::from_value(task, command_index);
        run_task_os_win_ps(&os_task, counter, output_yaml);
    } else if let Some(task) = task.get(&Value::String("os.cmd".to_string())) {
        let os_task: OSTask = OSTask::from_value(task, command_index);
        run_task_os_cmd(&os_task, counter, output_yaml);
    } else if let Some(task) = task.get(&Value::String("console.print".to_string())) {
        let console_task: ConsoleTask = ConsoleTask::from_value(task, command_index);
        run_task_console_print(&console_task, counter, output_yaml);
    } else if let Some(task) = task.get(&Value::String("loop.for".to_string())) {
        run_command_loop(command_index, task, counter, output_yaml);
    } else if let Some(task) = task.get(&Value::String("http.get".to_string())) {
        let http_task = HTTPTask::from_value(task, command_index);
        run_task_http_get(&http_task, counter, output_yaml);
    } else {
        print_error!("Command task not known in the input file");
    }
}

fn run_task_console_print(task: &ConsoleTask, counter: &mut Counters, output_yaml: &mut Value) {
    counter.total += 1;

    if task.execute {
        counter.executed += 1;
        task.display_message();

        let execute_command_output_value = Value::String(task.message.to_string());

        println!("{:?}", task.message.to_string());

        let path1 = format!("{}.out", task.output);
        let path2 = format!("{}.err", task.output);
        let path3 = format!("{}.sts", task.output);

        set_nested_value(output_yaml, &path1.as_str(), execute_command_output_value);
        set_nested_value(output_yaml, &path2.as_str(), Value::String("".to_string()));
        set_nested_value(output_yaml, &path3.as_str(), Value::String("0".to_string()));
        if task.debug_flag {
            print_debug!("### End: {:?}", output_yaml);
        }
    } else {
        print_warning!("SKIP: Not executing: {:?}", task);
        counter.skipped += 1;
    }
}

fn run_task_os_win_cmd(task: &OSTask, counter: &mut Counters, output_yaml: &mut Value) {
    counter.total += 1;
    if task.execute {
        counter.executed += 1;

        if cfg!(target_os = "windows") {
            task.display_message();
        } else {
            print_error!("ERROR: Not a windows operating system: {}", task.cmd);
            counter.errors += 1;
        }

        // execute the command
        let execute_command_output = std::process::Command::new("cmd")
            .arg("/c")
            .arg(task.cmd.clone())
            .output()
            .unwrap();

        let execute_command_output_value =
            Value::String(String::from_utf8_lossy(&execute_command_output.stdout).to_string());
        let execute_command_output_error =
            Value::String(String::from_utf8_lossy(&execute_command_output.stderr).to_string());
        let execute_command_output_status =
            Value::String((&execute_command_output.status).to_string());

        let path1 = format!("{}.out", task.output);
        let path2 = format!("{}.err", task.output);
        let path3 = format!("{}.sts", task.output);

        print_output!("{:?}", execute_command_output_value);

        set_nested_value(output_yaml, &path1.as_str(), execute_command_output_value);
        set_nested_value(output_yaml, &path2.as_str(), execute_command_output_error);
        set_nested_value(output_yaml, &path3.as_str(), execute_command_output_status);
        if task.debug_flag {
            print_debug!("### End: {:?}", output_yaml);
        }
    } else {
        print_warning!("SKIP: Not executing: {:?}", task);
        counter.skipped += 1;
    }
}

fn run_task_os_linux_cmd(task: &OSTask, counter: &mut Counters, output_yaml: &mut Value) {
    counter.total += 1;
    if task.execute {
        counter.executed += 1;

        if cfg!(target_os = "windows") {
            print_error!("ERROR: Not a windows operating system: {}", task.cmd);
            counter.errors += 1;
        } else {
            task.display_message();
        }

        // execute the command
        let execute_command_output = std::process::Command::new("sh")
            .arg("-c")
            .arg(task.cmd.clone())
            .output()
            .unwrap();

        let execute_command_output_value =
            Value::String(String::from_utf8_lossy(&execute_command_output.stdout).to_string());
        let execute_command_output_error =
            Value::String(String::from_utf8_lossy(&execute_command_output.stderr).to_string());
        let execute_command_output_status =
            Value::String((&execute_command_output.status).to_string());

        let path1 = format!("{}.out", task.output);
        let path2 = format!("{}.err", task.output);
        let path3 = format!("{}.sts", task.output);

        print_output!("{:?}", execute_command_output_value);

        set_nested_value(output_yaml, &path1.as_str(), execute_command_output_value);
        set_nested_value(output_yaml, &path2.as_str(), execute_command_output_error);
        set_nested_value(output_yaml, &path3.as_str(), execute_command_output_status);
        if task.debug_flag {
            print_debug!("### End: {:?}", output_yaml);
        }
    } else {
        print_warning!("SKIP: Not executing: {:?}", task);
        counter.skipped += 1;
    }
}

fn run_task_os_win_ps(task: &OSTask, counter: &mut Counters, output_yaml: &mut Value) {
    counter.total += 1;
    if task.execute {
        counter.executed += 1;

        if cfg!(target_os = "windows") {
            task.display_message();
        } else {
            print_error!("ERROR: Not a windows operating system: {}", task.cmd);
            counter.errors += 1;
        }

        // execute the command
        let execute_command_output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg(task.cmd.clone())
            .output()
            .unwrap();

        let execute_command_output_value =
            Value::String(String::from_utf8_lossy(&execute_command_output.stdout).to_string());
        let execute_command_output_error =
            Value::String(String::from_utf8_lossy(&execute_command_output.stderr).to_string());
        let execute_command_output_status =
            Value::String((&execute_command_output.status).to_string());

        let path1 = format!("{}.out", task.output);
        let path2 = format!("{}.err", task.output);
        let path3 = format!("{}.sts", task.output);

        print_output!("{:?}", execute_command_output_value);

        set_nested_value(output_yaml, &path1.as_str(), execute_command_output_value);
        set_nested_value(output_yaml, &path2.as_str(), execute_command_output_error);
        set_nested_value(output_yaml, &path3.as_str(), execute_command_output_status);
        if task.debug_flag {
            print_debug!("### End: {:?}", output_yaml);
        }
    } else {
        print_warning!("SKIP: Not executing: {:?}", task);
        counter.skipped += 1;
    }
}

fn run_task_os_cmd(task: &OSTask, counter: &mut Counters, output_yaml: &mut Value) {
    counter.total += 1;
    if task.execute {
        counter.executed += 1;

        task.display_message();

        // execute the command
        let execute_command_output: std::process::Output;
        if cfg!(target_os = "windows") {
            execute_command_output = std::process::Command::new("cmd")
                .arg("/c")
                .arg(task.cmd.clone())
                .output()
                .unwrap();
        } else {
            execute_command_output = std::process::Command::new("sh")
                .arg("-c")
                .arg(task.cmd.clone())
                .output()
                .unwrap();
        }
        let execute_command_output_value =
            Value::String(String::from_utf8_lossy(&execute_command_output.stdout).to_string());
        let execute_command_output_error =
            Value::String(String::from_utf8_lossy(&execute_command_output.stderr).to_string());
        let execute_command_output_status =
            Value::String((&execute_command_output.status).to_string());

        let path1 = format!("{}.out", task.output);
        let path2 = format!("{}.err", task.output);
        let path3 = format!("{}.sts", task.output);

        print_output!("{:?}", execute_command_output_value);

        set_nested_value(output_yaml, &path1.as_str(), execute_command_output_value);
        set_nested_value(output_yaml, &path2.as_str(), execute_command_output_error);
        set_nested_value(output_yaml, &path3.as_str(), execute_command_output_status);
        if task.debug_flag {
            print_debug!("### End: {:?}", output_yaml);
        }
    } else {
        print_warning!("SKIP: Not executing: {:?}", task);
        counter.skipped += 1;
    }
}

fn run_task_http_get(task: &HTTPTask, counter: &mut Counters, output_yaml: &mut Value) {
    counter.total += 1;
    if task.execute {
        counter.executed += 1;

        task.display_message();

        let client = reqwest::blocking::Client::new();
        let response = client.get(&task.url).send().unwrap();

        if response.status().is_success() {
            let response_text = response.text().unwrap();
            let response_status = "Success".to_string();
            let path1 = format!("{}.out", task.output);
            let path2 = format!("{}.sts", task.output);

            print_output!("{:?}", response_text);

            set_nested_value(output_yaml, &path1.as_str(), Value::String(response_text));
            set_nested_value(output_yaml, &path2.as_str(), Value::String(response_status));

            if task.debug_flag {
                print_debug!("### End: {:?}", output_yaml);
            }
        } else {
            print_error!("Error: {:?}", response.status());
            counter.errors += 1;
        }
    } else {
        print_warning!("SKIP: Not executing: {:?}", task);
        counter.skipped += 1;
    }
}

fn run_command_loop(
    command_index: &mut i32,
    task: &Value,
    counter: &mut Counters,
    output_yaml: &mut Value,
) {
    counter.total += 1;
    let execute: bool = task
        .get(&Value::String("execute".to_string()))
        .unwrap_or(&Value::Null)
        .as_bool()
        .unwrap_or(true);
    let debug_flag: bool = task
        .get(&Value::String("debug".to_string()))
        .unwrap_or(&Value::Null)
        .as_bool()
        .unwrap_or(false);

    if execute {
        counter.executed += 1;
        let default_name: String = format!("Command Index {}", command_index);
        //let default_output: String = format!("outs.output_{}", command_index);
        let name = task
            .get(&Value::String("name".to_string()))
            .unwrap_or(&Value::Null)
            .as_str()
            .unwrap_or(&default_name);
        let description = task
            .get(&Value::String("description".to_string()))
            .unwrap_or(&Value::Null)
            .as_str()
            .unwrap_or("None");

        let start = task
            .get(&Value::String("start".to_string()))
            .unwrap()
            .as_i64()
            .unwrap();
        let end = task
            .get(&Value::String("end".to_string()))
            .unwrap()
            .as_i64()
            .unwrap();
        let index_text = task
            .get(&Value::String("index".to_string()))
            .unwrap_or(&Value::Null)
            .as_str()
            .unwrap_or("index");

        let display_msg = format!("{}: {}", name, description);
        print_executing!("{}", display_msg);

        for i in start..end {
            let loop_tasks = task
                .get(&Value::String("tasks".to_string()))
                .unwrap()
                .as_sequence()
                .unwrap();
            if debug_flag {
                print_debug!("### Loop: {}", i);
                print_debug!("### Tasks: {:?}", loop_tasks);
            }
            for loop_task in loop_tasks.iter() {
                run_command_task(command_index, loop_task, counter, output_yaml);
            }
        }
    } else {
        print_warning!("SKIP: Not executing: {:?}", task);
        counter.skipped += 1;
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
    })
    .to_string()
}

fn get_nested_value<'a>(yaml_value: &'a Value, path: &str) -> Option<&'a Value> {
    //DEBUG: println!("yaml_value: {:?}", yaml_value);
    path.split('.')
        .map(str::trim)
        .try_fold(yaml_value, |current_value, key| current_value.get(key))
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
        env_vars
            .as_mapping_mut()
            .unwrap()
            .insert(Value::String(key), Value::String(value));
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
                } else {
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
                let env_var: String = environment_variables
                    .get(func_param_1)
                    .unwrap_or(&Value::String("".to_string()))
                    .as_str()
                    .unwrap()
                    .to_string();
                if env_var == "" {
                    eprintln!("Environment variable not found or empty: {}", func_param_1);
                }
                return env_var;
            }

            if function_name == "get_config" {
                let func_param_1 = modified_params.index(0);

                let config_variables = read_config_file();
                let config_var: String = config_variables
                    .get(func_param_1)
                    .unwrap_or(&Value::String("".to_string()))
                    .as_str()
                    .unwrap()
                    .to_string();
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

                let result = config_data
                    .iter()
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

// need to transform a string "root.level1.name: 'demo'" a Value
fn set_nested_value<'a>(yaml_value: &'a mut Value, path: &str, value: Value) {
    if yaml_value.is_null() {
        *yaml_value = Value::Mapping(serde_yaml::Mapping::new());
    }
    let mut current_value = yaml_value;

    let mut keys = path.split('.');
    let last_key = keys.next_back().unwrap();
    for key in keys {
        current_value = current_value
            .as_mapping_mut()
            .unwrap()
            .entry(Value::String(key.to_string()))
            .or_insert(Value::Mapping(serde_yaml::Mapping::new()));
    }
    current_value
        .as_mapping_mut()
        .unwrap()
        .insert(Value::String(last_key.to_string()), value);
}
