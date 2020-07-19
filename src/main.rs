mod container;
mod custom_constants;

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


fn main() {
    let args: Vec<String> = env::args().collect();
    let action: &str = args.get(1).expect("script <help>");

    if action == "help" {
        print_help_text();
        return
    }

    let index_number: &str = args.get(2).expect("script <help>");
    let index_number: String = index_number.to_string();
    let index_number: u8 = index_number.parse::<u8>().unwrap() -1 ;

    let continer: Container = Container::new(index_number.to_owned());

    let result = match action {
        "build" => continer.build(),
        "run" => continer.run(),
        "start" => continer.start(),
        "stop" => continer.stop(),
        "remove" => continer.remove(),
        _ => {
            println!("{0}", "script <help>");
            Ok("Help".to_owned())
        }
    };

    result.map_err(|err| println!("Some error occured {}", err)).ok();
}
