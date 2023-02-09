use std::fs;
use std::fs::{File, FileTimes};
use std::io::Result;
use std::path::Path;
use std::process;
use std::time::SystemTime;

use chrono::{DateTime, Local, TimeZone};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    file_paths: Option<Vec<String>>,

    #[arg(short, long, action=ArgAction::SetTrue, help="change only the access time")]
    access: Option<bool>,

    #[arg(short='c', long="no-create", action=ArgAction::SetTrue, help="do not create any files")]
    no_create: Option<bool>,

    #[arg(
        short,
        long,
        help = "parse STRING and use it instead of current time, e.g: '2001-01-01 12:02:03'"
    )]
    date: Option<String>,

    #[arg(short, long, action=ArgAction::SetTrue, help="change only the modification time")]
    modification: Option<bool>,

    #[arg(short, long, help = "use this file's times instead of current time")]
    reference: Option<String>,
}

pub struct Executor {
    cli: Cli,
    acc_time: FileTimes,
    mod_time: FileTimes,
}

impl Executor {
    pub fn with_cli(cli: Cli) -> Self {
        Executor { 
            cli,
            acc_time: FileTimes::new().set_accessed(SystemTime::now()),
            mod_time: FileTimes::new().set_modified(SystemTime::now()),
        }
    }

    pub fn execute(&mut self) {
        if let Some(file_paths) = self.cli.file_paths.clone().as_ref() {
            self.get_times();
            match self.execute_files(file_paths) {
                Ok(()) => return,
                Err(err) => {
                    exit(format!("Wrouch error: {}", err.to_string()), 1);
                }
            }
        } else {
            exit(
                format!(
                "No file paths to be found, type 'wrouch --help/-h' to learn how to use wrouch."),
                1,
            );
        };
    }

    fn execute_files(&self, file_paths: &[String]) -> Result<()> {
        let mut file: File;
        for path in file_paths {
            let p = Path::new(path);
            if !p.exists() {
                if Some(false) == self.cli.no_create {
                    file = File::create(p)?;
                } else {
                    continue;
                }
            } else {
                file = File::open(path)?
            }
            if let Err(err) = self.set_times(&file) {
                exit(format!("Set time error: {}", err.to_string()), 1);
            }
        }
        Ok(())
    }

    fn get_times(&mut self) {
        if let Some(date) = self.cli.date.as_ref() {
            let datetime: DateTime<Local>;
            match Local.datetime_from_str(date.as_str(), "%Y-%m-%d %H:%M:%S") {
                Ok(dt) => datetime = dt.into(),
                Err(_) => match Local.datetime_from_str(date.as_str(), "%Y-%m-%d") {
                    Ok(dt) => datetime = dt.into(),
                    Err(_) => {
                        exit(format!("Cannot parse datetime from {}", date), 1);
                        process::exit(1);
                    }
                },
            }
            self.acc_time = self.acc_time.set_accessed(datetime.into());
            self.mod_time = self.mod_time.set_modified(datetime.into());
        } else if let Some(refer) = self.cli.reference.as_ref() {
            let meta = fs::metadata(Path::new(refer));
            match meta {
                Ok(md) => {
                    match md.accessed() {
                        Ok(t) => self.acc_time = self.acc_time.set_accessed(t),
                        Err(err) => exit(
                            format!("Get time of reference file {} error: {}", refer, err.to_string()),
                            1,
                        ),
                    }
                    match md.modified() {
                        Ok(t) => self.mod_time = self.mod_time.set_modified(t),
                        Err(err) => exit(
                            format!("Get time of reference file {} error: {}", refer, err.to_string()),
                            1,
                        ),
                    }
                }
                Err(err) => {
                    exit(
                        format!("failed to get attributes of '{}': {}", refer, err.to_string()),
                        1,
                    );
                }
            }
        }
    }

    fn set_times(&self, file: &File) -> Result<()> {
        if Some(true) == self.cli.modification {
            file.set_times(self.mod_time)?;
        } else if Some(true) == self.cli.access {
            file.set_times(self.acc_time)?;
        } else {
            file.set_times(self.acc_time)?;
            file.set_times(self.mod_time)?;
        }
        Ok(())
    }
}

fn exit(msg: String, code: i32) {
    println!("{:?}", msg);
    process::exit(code);
}
