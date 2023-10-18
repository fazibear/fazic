use std::collections::HashMap;

pub trait FileSystem {
    fn dir(&self) -> Result<Vec<String>, String>; 
    fn load(&self, filename: &str) -> Result<String, String>;
    fn save(&mut self, filename: &str, program: String) -> Result<(), String>;
}

pub struct MemoryFileSystem {
    files: HashMap<String, String>
}

impl MemoryFileSystem {
    pub fn new() -> MemoryFileSystem {
        MemoryFileSystem { files: HashMap::new() }
    }
}

impl FileSystem for MemoryFileSystem {
    fn dir(&self) -> Result<Vec<String>, String> {
        Ok(self.files.keys().map(|s| s.to_string()).collect())
    }

    fn load(&self, filename: &str) -> Result<String, String> {
        match self.files.get(filename) {
            Some(lines) => Ok(lines.clone()),
            None => Err(format!("File {} not found", filename))
        }
    }

    fn save(&mut self, filename: &str, program: String) -> Result<(), String> {
        let _ = self.files.insert(filename.to_string(), program);
        Ok(())
    }
} 
