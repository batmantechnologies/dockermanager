
use std::process::{ Command };
use crate::custom_constants::DOCKERFILE;


pub struct Container {
    pub name: String
}

impl Container {

    pub fn new(name: String) ->  Container {
        Container { name: name }
    }

    pub fn build(&self) -> Result<String, String> {

        let data = &format!("docker build -t postgres:latest -<<EOF{0}EOF", DOCKERFILE);
        let build = Command::new("/bin/bash")
                    .args(&["-c", data])
                    .status()
                    .expect("failed to execute process");
        if  build.success() {
            Ok("success".to_owned())
        } else {
            Err(build.code().unwrap().to_string())
        }
    }

    pub fn start(&self) -> Result<String, String> {
        let build = Command::new("/bin/bash")
                    .args(&["-c", &format!("docker start {0}", self.name)])
                    .status()
                    .expect("failed to execute process");

        if  build.success() {
            Ok("success".to_owned())
        } else {
            Err(build.code().unwrap().to_string())
        }
    }

    fn check_running(&self) -> Option<String> {

        let result = Command::new("/bin/bash")
                .args(&["-c", &format!("docker ps|grep {0}", self.name)])
                .output()
                .expect("failed to execute process");

        if result.stdout.len() > 0 {
            Some("success".to_owned())
        } else {
            None
        }
    }

    fn check_stopped(&self) -> Option<String> {

        let result = Command::new("/bin/bash")
                .args(&["-c", &format!("docker ps -a|grep {0}", self.name)])
                .output()
                .expect("failed to execute process");

        if result.stdout.len() > 0 {
            Some("success".to_owned())
        } else {
            None
        }
    }

    pub fn stop(&self) -> Result<String, String> {
        let result = Command::new("/bin/bash")
                    .args(&["-c", &format!("docker stop {0}", self.name)])
                    .status()
                    .expect("failed to execute process");
        if  result.success() {
            Ok("success".to_owned())
        } else {
            Err(result.code().unwrap().to_string())
        }
    }

    pub fn remove(&self) -> Result<String, String> {
        let result = Command::new("/bin/bash")
                    .args(&["-c", &format!("docker remove {0}", self.name)])
                    .status()
                    .expect("failed to execute process");
        if  result.success() {
            Ok("success".to_owned())
        } else {
            Err(result.code().unwrap().to_string())
        }
    }

    pub fn run(&self) -> Result<String, String> {
        if let Some(_) = self.check_running() {
            println!("Container is already running..");
        } else if let Some(_) = self.check_stopped() {
            self.start().expect("Process Failed to Start");
            println!("Container has stopped and srarted.");
        } else {
            self.build().expect("Process Failed to Build");
            let cmd = &format!("docker run -d -p 5432:5432 --name {0} postgres:latest", self.name);
            let result = Command::new("/bin/bash")
                .args(&["-c", cmd])
                .status()
                .expect("failed to execute process");
            println!("New Container is created. and running");
            if  result.success() {
                println!("New Container is started");
            } else {
                println!("Problem in container running");
            }
        }
        Ok("success".to_owned())
    }
}
