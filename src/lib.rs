use std::path::Path;

pub struct FileManager {
    filename: String,
}

impl FileManager {
    pub fn new(filename: String) -> FileManager {
        FileManager { filename: filename }
    }

    pub fn create_file(&self) -> Result<String, String> {
        let path = Path::new(&self.filename);
        if path.exists() {
            return Err(String::from("File already exists"));
        }
        Ok(self.filename.clone())
    }
}
