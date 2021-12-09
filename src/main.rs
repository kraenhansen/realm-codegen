//! Generates code binding from the MongoDB Realm Core C++ API (codified in a subset of [WebIDL]) with other platforms,
//! using rules codified in [handlebars] templates.
//!
//! [WebIDL]: https://webidl.spec.whatwg.org/
//! [handlebars]: https://handlebarsjs.com/

use std::path::PathBuf;

mod codegen;

fn main() {
    println!("Realm Codegen\n");
    // Read configuration
    let config_path_string = std::env::args().nth(1).expect("expected a config path");
    let config_path = PathBuf::from(config_path_string);
    let config = codegen::config::read_config(&config_path);
    // Read output path
    let output_path_string = std::env::args().nth(2).expect("expected an output path");
    let output_path = PathBuf::from(output_path_string);

    match config {
        Ok(c) => {
            let result = codegen::generator::generate(c, &output_path);
            match result {
                Ok(_) => println!("All done ..."),
                Err(e) => {
                    println!("Failed to generate binding: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("Failed to read config: {}", e);
            std::process::exit(1);
        }
    };
}
