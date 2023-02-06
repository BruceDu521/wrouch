use wrench::FileManager;

use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    file_names: Option<Vec<String>>,

    #[arg(short, long, action=ArgAction::SetTrue, help="change only the access time")]
    access: Option<String>,

    #[arg(short='c', long="no-create", action=ArgAction::SetTrue, help="do not create any files")]
    no_create: Option<bool>,

    #[arg(short, long, help="parse STRING and use it instead of current time")]
    date: Option<String>,

    #[arg(short, long, action=ArgAction::SetTrue, help="change only the modification time")]
    modification: Option<String>
}

pub struct Executor {
    cli: Cli,
}

impl Executor {
    pub fn with_cli(cli: Cli) -> Self {
        Executor { cli: cli }
    }

    pub fn execute(&self) {
        if let Some(file_names) = self.cli.file_names.as_deref() {
            self.create_files(file_names);
        } else {
            println!("Nothing happened.")
        };
    }

    fn create_files(&self, file_names: &[String]) {
        for name in file_names {
            let manager = FileManager::new(name.to_owned());

            match manager.create_file() {
                Ok(file) => println!("Created file: {:?}", file),
                Err(err) => println!("Wrench error: {:?}", err),
            }
        }
    }
}