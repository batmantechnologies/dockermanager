mod container;
mod custom_constants;

use std::env;
use custom_constants::DATABASE_CONTAINERS;
use container::Container;

const HELP_TEXT: &str = r#"
        ------------------- HELP----------------

        script <action> <container_name>

        script build <container_name>
        script run <container_name>
        script stop <container_name>
        script start <container_name>
        script remove <container_name>

        Container name :
            1. prd_service_db

        ------------------- HELP----------------
        "#;


fn main() {
    let args: Vec<String> = env::args().collect();
    let action: &str = args.get(1).expect(HELP_TEXT);

    if action == "help" {
        println!("{0}", HELP_TEXT);
        return
    }

    let name: &str = args.get(2).expect(HELP_TEXT);

    if !DATABASE_CONTAINERS.contains(&name) {
        println!("{0}", HELP_TEXT);
    }

    let continer: Container = Container::new(name.to_owned());

    let result = match action {
        "build" => continer.build(),
        "run" => continer.run(),
        "start" => continer.start(),
        "stop" => continer.stop(),
        "remove" => continer.remove(),
        _ => {
            println!("{0}", HELP_TEXT);
            Ok("Help".to_owned())
        }
    };
}
