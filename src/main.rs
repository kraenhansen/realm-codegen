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
    let mut config_path = PathBuf::from(config_path_string);
    if config_path.is_dir() {
        let config_file_path = config_path.join("config.yml");
        if config_file_path.is_file() {
            config_path = config_file_path;
        }
    }
    let config = codegen::config::read_config(&config_path);
    // Read output path
    let output_path_string = std::env::args().nth(2).expect("expected an output path");
    let output_path = PathBuf::from(output_path_string);

    match config {
        Ok(c) => {
            let result = codegen::generator::generate(c, &output_path);
            match result {
                Ok(_) => println!("All done ..."),
                Err(e) => println!("Failed to generate binding: {}", e),
            }
        }
        Err(e) => println!("Failed to read config ({}): {}", config_path.display(), e),
    };
}
