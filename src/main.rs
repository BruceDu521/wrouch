use std::env;
use std::process;

use wrench::FileManager;

fn main() {
    // args[0] 是执行文件的路径；
    // env::current_dir() 是执行者的当前目录；
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
