use std::process::{ Command };
use std::env;

const DOCKERFILE: &str = include_str!("dockers/PostgreSql");

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let com = run_command(&args).await;
}

async fn run_command(args: &Vec<String>) {
    if args.contains(&"build".to_owned())  {
        build_only().await
    } else if args.contains(&"userdatabase".to_owned()) {
        println!("Super")
    } else {
        help().await
    }
}

async fn build_only() {

    let data = format!("docker build -t postgres:latest -<<EOF{0}EOF", DOCKERFILE);

    println!(" -----------------1--------- hello ---------------");
    println!("{}", data);

    let build = Command::new("/bin/bash")
                .arg("-c")
                .arg(data)
                .spawn()
                .expect("failed to execute process");
}

async fn help() {
    println!("script <database_name>");
    println!("database list as below");
}
