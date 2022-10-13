use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::io::{Result, Error, ErrorKind};


pub struct FileManager {
    file_path: PathBuf,
}

impl FileManager {
    pub fn new(filename: String) -> FileManager {
        let path = Path::new(&filename);
        let file_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            Path::new(&env::current_dir().unwrap()).join(path)
        };
        FileManager { 
            file_path,
        }
    }

    pub fn create_file(&self) -> Result<String> {
        let path_str = self.file_path.as_path().display().to_string();
        if self.file_path.exists() && !self.file_path.is_dir() {
            return Err(
                Error::new(
                    ErrorKind::Other, 
                    format!("File {:?} already exists", path_str)
                )
            );
        }
        File::create(self.file_path.as_path())?;
        Ok(path_str)
    }
}
