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
                    exit(format!("Wrouch error: {:?}", err), 1);
                }
            }
        } else {
            exit(format!(
                "No file paths to be found, type 'wrouch --help/-h' to learn how to use wrouch."), 
                1);
        };
    }

    fn execute_files(&self, file_paths: &[String]) -> Result<()> {
        let mut file: File;
        let (acc_time, mod_time) = self.get_time();
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
            if let Err(err) = self.set_times(&file, acc_time, mod_time) {
                exit(format!("Set time error: {:?}", err), 1);
            }
        }
        Ok(())
    }

    fn get_time(&self) -> (FileTimes, FileTimes) {
        let mut acc_time = FileTimes::new().set_modified(SystemTime::now());
        let mut mod_time = FileTimes::new().set_modified(SystemTime::now());
        if let Some(date) = self.cli.date.as_ref() {
            let datetime: DateTime<Local>;
            match Local.datetime_from_str(date.as_str(), "%Y-%m-%d %H:%M:%S") {
                Ok(dt) => datetime = dt.into(),
                Err(_) => match Local.datetime_from_str(date.as_str(), "%Y-%m-%d") {
                    Ok(dt) => datetime = dt.into(),
                    Err(_) => {
                        exit(format!("Cannot parse datetime from {:?}", date), 1);
                        process::exit(1);
                    }
                },
            }
            acc_time = acc_time.set_accessed(datetime.into());
            mod_time = mod_time.set_modified(datetime.into());
        } else if let Some(refer) = self.cli.reference.as_ref() {
            let meta = fs::metadata(Path::new(refer));
            match meta {
                Ok(md) => {
                    match md.accessed() {
                        Ok(t) => acc_time = mod_time.set_accessed(t),
                        Err(err) => exit(format!("Get time of reference file {:?} error: {:?}", refer, err), 1)
                    }
                    (md.accessed().unwrap());
                    mod_time = mod_time.set_modified(md.modified().unwrap());
                }
                Err(err) => {
                    exit(format!("failed to get attributes of '{:?}': {:?}", refer, err), 1);
                }
            }
        }
        (acc_time, mod_time)
    }

    fn set_times(&self, file: &File, acc_time: FileTimes, mod_time: FileTimes) -> Result<()> {
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

fn exit(msg: String, code: i32) {
    println!("{:?}", msg);
    process::exit(code);
}
