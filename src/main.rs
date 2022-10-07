use std::env;
use std::process;

use wrench::FileManager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: wrench [FILENAME] [OPTION]");
        process::exit(1);
    }

    let manager = FileManager::new(args[1].to_owned());

    match manager.create_file() {
        Ok(file) => println!("Created file: {:?}", file),
        Err(err) => println!("Wrench error: {:?}", err),
    }
}
