use fazic::file_system::FileSystem;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FAZIC_FS: &str = "../fazic_fs";

pub struct DiskFileSystem {}

impl DiskFileSystem {
    pub fn new() -> DiskFileSystem {
        DiskFileSystem {}
    }
}

impl FileSystem for DiskFileSystem {
    fn dir(&self) -> Result<Vec<String>, String> {
        let with_path = format!("{}/", FAZIC_FS);
        let paths = fs::read_dir(&with_path).unwrap();

        let names = paths
            .map(|entry| {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                let file_name = entry_path.file_name().unwrap();
                let file_name_as_str = file_name.to_str().unwrap();
                let file_name_as_string = String::from(file_name_as_str);
                file_name_as_string
            })
            .collect::<Vec<String>>();
        Ok(names)
    }

    fn load(&self, filename: &str) -> Result<String, String> {
        let with_path = format!("{}/{}", FAZIC_FS, filename);
        let path = Path::new(&with_path);
        let mut result = String::new();

        match File::open(path) {
            Ok(mut file) => match file.read_to_string(&mut result) {
                Ok(_) => Ok(result),
                _ => Err("NOT FOUND".to_string()),
            },
            _ => Err("NOT_FOUND".to_string()),
        }
    }

    fn save(&mut self, filename: &str, program: String) -> Result<(), String> {
        let with_path = format!("{}/{}", FAZIC_FS, filename);
        let path = Path::new(&with_path);

        match File::create(path) {
            Ok(mut file) => match file.write_all(program.as_bytes()) {
                Ok(_) => Ok(()),
                _ => Err("NOT SAVED".to_string()),
            },
            _ => Err("NOT SAVED".to_string()),
        }
    }
}
