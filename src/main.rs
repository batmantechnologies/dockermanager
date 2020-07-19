mod container;
mod custom_constants;
use std::fs;
use std::path::Path;
use std::env;
use container::Container;

use crate::custom_constants::{INDEX_SELECTION};

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
    for element in INDEX_SELECTION.iter().enumerate() {
        println!("{}. {}", element.0+1, element.1)
    }
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
        } else {
            println!("Not a DockerFile - {}", docker_dir.to_str().unwrap());
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

    println!("{:?}", docker_files);

    //for entry in fs::read_dir(path) {
    //    let dir = entry;
    //    println!("{:?}", dir.path());
    //}


    //if action == "help" {
    //    print_help_text();
    //    return
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
