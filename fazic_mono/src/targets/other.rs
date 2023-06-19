use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FAZIC_FS: &str = "../fazic_fs";

pub fn set_main_loop_callback<F>(mut f: F)
where
    F: FnMut(),
{
    loop {
        f();
    }
}
pub fn dir() -> Vec<String> {
    let mut result = vec![];

    let with_path = format!("{}/", FAZIC_FS);

    let paths = fs::read_dir(&with_path).unwrap();

    for path in paths {
        let file = path.unwrap().file_name();
        let mut name = file.to_string_lossy().to_string();
        let len = name.len() - 4;

        if name.ends_with(".bas") {
            name.truncate(len);
            result.push(format!("LOAD \"{}\"", name))
        };
    }

    result
}

pub fn load(name: &String) -> Result<String, String> {
    let with_path = format!("{}/{}.bas", FAZIC_FS, name);
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

pub fn save(name: &String, body: &String) -> Result<String, String> {
    let with_path = format!("{}/{}.bas", FAZIC_FS, name);
    let path = Path::new(&with_path);

    match File::create(path) {
        Ok(mut file) => match file.write_all(body.as_bytes()) {
            Ok(_) => Ok("OK".to_string()),
            _ => Err("NOT SAVED".to_string()),
        },
        _ => Err("NOT SAVED".to_string()),
    }
}
