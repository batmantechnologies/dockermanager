use std::fs::File;
use std::io::prelude::*;
use std::process::{ Command };
use crate::custom_constants::{DOCKER_FILES};
use std::collections::HashMap;

#[derive(Debug)]
pub enum InputCommand {
    Build(Vec<i8>),
    Run(Vec<i8>),
    Start(Vec<i8>),
    Stop(Vec<i8>),
    Remove(Vec<i8>)
}

#[derive(Debug)]
pub struct Commander {
    command: InputCommand,
    name_path_hash: HashMap<String, String>,
    index_name_hash: HashMap<i8, String>
}

impl Commander {

    pub fn new(command: InputCommand,
               name_path_hash: HashMap<String, String>,
               index_name_hash: HashMap<i8, String>) ->  Commander {

        Commander {
            command: command,
            name_path_hash: name_path_hash,
            index_name_hash: index_name_hash
         }
    }

    pub fn execute(&self) ->  Result<String, String> {

        let result = match &self.command {
            InputCommand::Build(container_index) => {self.build(container_index.to_vec())},
            InputCommand::Run(container_index) =>  {self.build(container_index.to_vec())},
            InputCommand::Start(container_index) => {self.build(container_index.to_vec())},
            InputCommand::Stop(container_index) => {self.build(container_index.to_vec())},
            InputCommand::Remove(container_index) => {self.build(container_index.to_vec())}
        };
        Ok("Done".to_string())
    }

    fn get_cmd_file(&self, container_index: Vec<i8>) -> Vec<(String, String)> {

        let mut name_and_file: Vec<(String, String)> = Vec::new();

        for index in container_index {
            let index_name = self.index_name_hash.get(&index).unwrap();
            let file_path = self.name_path_hash.get(index_name).unwrap();
            name_and_file.push((index_name.into(), file_path.to_owned()));
        }
        name_and_file
    }

    fn build(&self, container_index: Vec<i8>) -> Result<String, String> {

        let name_file: Vec<(String, String)> = self.get_cmd_file(container_index);

        for record in name_file {


            let data = &format!("docker build -t {0}:latest - < {1}", record.0, record.1);
    
            let build = Command::new("/bin/bash")
                        .args(&["-c", data])
                        .status()
                        .expect("failed to execute process");
            if  build.success() {
                println!("{}", "success".to_owned());
            } else {
                println!("{}", build.code().unwrap().to_string());
            }
        }
        Ok("Done".to_owned())
    }

  //pub fn start(&self) -> Result<String, String> {
  //    let build = Command::new("/bin/bash")
  //                .args(&["-c", &format!("docker start {0}", self.name)])
  //                .status()
  //                .expect("failed to execute process");

  //    if  build.success() {
  //        Ok("success".to_owned())
  //    } else {
  //        Err(build.code().unwrap().to_string())
  //    }
  //}

    //fn check_running(&self) -> Option<String> {
//
    //    let result = Command::new("/bin/bash")
    //            .args(&["-c", &format!("docker ps|grep {0}", self.name)])
    //            .output()
    //            .expect("failed to execute process");
//
    //    if result.stdout.len() > 0 {
    //        Some("success".to_owned())
    //    } else {
    //        None
    //    }
    //}
//
    //fn check_stopped(&self) -> Option<String> {
//
    //    let result = Command::new("/bin/bash")
    //            .args(&["-c", &format!("docker ps -a|grep {0}", self.name)])
    //            .output()
    //            .expect("failed to execute process");
//
    //    if result.stdout.len() > 0 {
    //        Some("success".to_owned())
    //    } else {
    //        None
    //    }
    //}
//
    //pub fn stop(&self) -> Result<String, String> {
    //    let result = Command::new("/bin/bash")
    //                .args(&["-c", &format!("docker stop {0}", self.name)])
    //                .status()
    //                .expect("failed to execute process");
    //    if  result.success() {
    //        Ok("success".to_owned())
    //    } else {
    //        Err(result.code().unwrap().to_string())
    //    }
    //}
//
    //pub fn remove(&self) -> Result<String, String> {
    //    self.stop().map_err(|err| println!("Container not running {}", err)).ok();
    //    let result = Command::new("/bin/bash")
    //                .args(&["-c", &format!("docker rm {0}", self.name)])
    //                .status()
    //                .expect("failed to execute process");
    //    if  result.success() {
    //        Ok("success".to_owned())
    //    } else {
    //        Err(result.code().unwrap().to_string())
    //    }
    //}
//
    //pub fn run(&self) -> Result<String, String> {
    //    if let Some(_) = self.check_running() {
    //        println!("Container is already running..");
    //    } else if let Some(_) = self.check_stopped() {
    //        self.start().expect("Process Failed to Start");
    //        println!("Container has stopped and srarted.");
    //    } else {
    //        self.build().expect("Process Failed to Build");
    //        let cmd = &format!("docker run -d -p 5432:5432 --name {0} postgres:latest", self.name);
    //        let result = Command::new("/bin/bash")
    //            .args(&["-c", cmd])
    //            .status()
    //            .expect("failed to execute process");
    //        println!("New Container is created. and running");
    //        if  result.success() {
    //            println!("New Container is started");
    //        } else {
    //            println!("Problem in container running");
    //        }
    //    }
    //    Ok("success".to_owned())
    //}
}
