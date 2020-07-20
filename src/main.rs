mod container;
mod custom_constants;
use std::fs;
use std::path::Path;
use std::env;
use container::{Commander, InputCommand};
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

//extern crate regex;
use regex::Regex;

const HELP_TEXT: &str = r#"
        ------------------- HELP----------------

        script <action> <container_index>

        script build <container_index>
        script run <container_index>
        script stop <container_index>
        script start <container_index>
        script remove <container_index>

        Select container by its index :
        ------------------- HELP----------------
        "#;

fn print_help_text() {
    println!("{0}", HELP_TEXT);
}

fn docker_finder(target_directory: &Path, docker_files: &mut Vec<String>) {

    for docker_dir in fs::read_dir(&target_directory).unwrap() {
        let docker_dir = docker_dir.unwrap().path();
        let last_section: String = docker_dir.file_stem().unwrap().to_str().unwrap().into();

        if docker_dir.is_dir() {
            docker_finder(&docker_dir, docker_files);
        } else  if last_section.to_lowercase().contains("dockerfile") {
            let docker_dir = docker_dir.to_str().unwrap();
            docker_files.push(docker_dir.to_string());
        }
    };
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &str = args.get(1).expect("script <pah to find dockerfiles>");

    let target_directory = Path::new(path);
    let mut docker_files: Vec<String> = Vec::new();

    if !target_directory.is_dir() {
        panic!("Given path is not the directory!")
    };

    docker_finder(&target_directory, &mut docker_files);

    let mut name_path_hash: HashMap<String, String> = HashMap::new();
    let mut index_name_hash: HashMap<i8, String> = HashMap::new();

    let compiled_regex = Regex::new("[^0-9A-Za-z]").unwrap();

    for path_string in docker_files {

        let dock_name: Vec<&str> = path_string.split("/").collect();
        let end = dock_name.len();
        let start = end - 4;

        let dock_name: &str = &dock_name[start .. end].join("_");

        let stripped_dock_name: String = compiled_regex.replace_all(&dock_name, "_").into_owned();
        name_path_hash.insert(stripped_dock_name, path_string.to_string());
    }

    for (index, (key, value)) in name_path_hash.iter().enumerate() {
        index_name_hash.insert(index as i8 +1, key.to_string());
    }

    print_help_text();

    for (index, value) in &index_name_hash {
        println!("{}:  {}", index, value);
    }

    let mut input_command = String::new();
    let mut container_indexes = String::new();

    while input_command.len() == 0 {
        println!("Enter your command");
        io::stdin().read_line(&mut input_command).unwrap();
        input_command = input_command.trim().to_string();
    }

    while container_indexes.len() == 0 {
        println!("Enter indexes separated by comma");
        io::stdin().read_line(&mut container_indexes).unwrap();
        container_indexes = container_indexes.trim().to_string();
    }

    let mut container_indexes_array: Vec<i8> = Vec::new();
    for index in container_indexes.split(",").collect::<Vec<&str>>() {
        let in_dex = index.parse::<i8>().unwrap_or(0);
        if (in_dex as usize) <= index_name_hash.len() && in_dex != 0 {
            container_indexes_array.push(in_dex);
        }
    }

    println!("{:?}", container_indexes_array);

    let user_command: InputCommand = match input_command.as_str() {
        "build" => InputCommand::Build(container_indexes_array),
        "run" => InputCommand::Run(container_indexes_array),
        "start" => InputCommand::Start(container_indexes_array),
        "stop" => InputCommand::Stop(container_indexes_array),
        "remove" => InputCommand::Remove(container_indexes_array),
        _ => panic!("Noting entred"),
    };

    println!("{:?}",  user_command);


    //if action == "help" {
    //    print_help_text();
    //    return,
    //}
//
    //let index_number: &str = args.get(2).expect("script <help>");
    //let index_number: String = index_number.to_string();
    //let index_number: u8 = index_number.parse::<u8>().unwrap() -1 ;
//
    //let continer: Container = Container::new(index_number.to_owned());
//
    //let result = match action {
    //    "build" => continer.build(),
    //    "run" => continer.run(),
    //    "start" => continer.start(),
    //    "stop" => continer.stop(),
    //    "remove" => continer.remove(),
    //    _ => {
    //        println!("{0}", "script <help>");
    //        Ok("Help".to_owned())
    //    }
    //};
//
    //result.map_err(|err| println!("Some error occured {}", err)).ok();
}
