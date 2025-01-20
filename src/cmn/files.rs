use std::{
    fs::{self, File},
    path::Path,
};

pub fn create_new_with_all_dir(path: &String) -> Result<File, std::io::Error> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    File::create_new(path)
}

pub fn create_with_all_dir(path: &String) -> Result<File, std::io::Error> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)
}

pub fn remove_if_present(path: &String) -> Result<(), std::io::Error> {
    if Path::new(path).exists() {
        return fs::remove_file(path);
    }
    Ok(())
}

pub fn list_dir(path: &String) -> Result<Option<Vec<String>>, std::io::Error> {
    let path = Path::new(path);
    if !path.is_dir() {
        return Ok(None);
    }
    let mut list = Vec::new();
    for entry in fs::read_dir(path)? {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    list.push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
    }
    return Ok(Some(list));
}
