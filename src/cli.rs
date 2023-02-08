use std::fs::{File, FileTimes};
use std::path::Path;
use std::process;
use std::io::Result;
use std::time::SystemTime;

use clap::{Parser, ArgAction};
use chrono::{DateTime, Local};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    file_paths: Option<Vec<String>>,

    #[arg(short, long, action=ArgAction::SetTrue, help="change only the access time")]
    access: Option<bool>,

    #[arg(short='c', long="no-create", action=ArgAction::SetTrue, help="do not create any files")]
    no_create: Option<bool>,

    #[arg(short, long, help="parse STRING and use it instead of current time, e.g: '2001-01-01 12:02:03'")]
    date: Option<String>,

    #[arg(short, long, action=ArgAction::SetTrue, help="change only the modification time")]
    modification: Option<bool>
}

pub struct Executor {
    cli: Cli,
}

impl Executor {
    pub fn with_cli(cli: Cli) -> Self {
        Executor { cli }
    }

    pub fn execute(&self) {
        if let Some(file_paths) = self.cli.file_paths.as_deref() {
            match self.execute_files(file_paths) {
                Ok(()) => return,
                Err(err) => {
                    println!("Wrench error: {:?}", err);
                    process::exit(1);
                }
            }
        } else {
            println!("No file paths to be found, type 'wrench --help/-h' to learn how to use wrench.");
            process::exit(1);
        };
    }

    fn execute_files(&self, file_paths: &[String]) -> Result<()> {
        let mut file: File;
        let datetime = self.parse_time();
        let sys_time: SystemTime;
        let mut mod_time = FileTimes::new();
        let mut acc_time = FileTimes::new();
        if let Some(dt) = datetime.to_owned() {
            sys_time = SystemTime::from(dt);
        } else {
            sys_time = SystemTime::now();
        }
        mod_time = mod_time.set_modified(sys_time);
        acc_time = acc_time.set_accessed(sys_time);
        for path in file_paths {
            let p = Path::new(path);
            if !p.exists() {
                if Some(false) == self.cli.no_create {
                    File::create(p)?;
                }
                continue;
            } else {
                file = File::open(path)?
            }
            if let Err(err) = self.set_times(&file, acc_time, mod_time) {
                println!("Set time error: {:?}", err);
                process::exit(1);
            }
        }
        Ok(())
    }

    fn parse_time(&self) -> Option<DateTime<Local>> {
        if let Some(date) = self.cli.date.as_ref() {
            let datetime: DateTime<Local>;
            match DateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M:%S") {
                Ok(dt) => datetime = dt.into(),
                Err(_) => {
                    match DateTime::parse_from_str(date.as_str(), "%Y-%m-%d") {
                        Ok(dt) => datetime = dt.into(),
                        Err(_) => {
                            println!("Cannot parse datetime from {:?}", date);
                            process::exit(1);
                        }
                    }
                }
            }
            Some(datetime)
        } else {
            None
        }
    }

    fn set_times(&self, file: &File, acc_time: FileTimes, mod_time: FileTimes) -> Result<()> {
        println!("{:?}", mod_time);
        if Some(true) == self.cli.modification {
            file.set_times(mod_time)?;
        } else if Some(true) == self.cli.access {
            file.set_times(acc_time)?;
        } else {
            file.set_times(mod_time)?;
            file.set_times(acc_time)?;
        }
        Ok(())
    }
}