use crate::core::error::{AliasError, AliasErrorCode};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

pub fn create_if_absent(path: &String) -> Result<File, AliasError> {
    let path_buf = PathBuf::from_str(path).unwrap();
    if path_buf.exists() {
        return File::open(&path_buf).map_err(|e| AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("open file fail :: {} :: {}", path_buf.display(), e),
        });
    }
    create(path)
}

pub fn create(path: &String) -> Result<File, AliasError> {
    let path_buf = PathBuf::from_str(path).unwrap();
    if let Some(parent) = path_buf.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("create dir fail :: {} :: {}", parent.display(), e),
            });
        }
    }
    return File::create(&path_buf).map_err(|e| AliasError {
        err: AliasErrorCode::Unkonw,
        msg: format!("create file fail :: {} :: {}", path_buf.display(), e),
    });
}

pub fn list_dir(path: &String) -> Result<Option<Vec<String>>, AliasError> {
    let path_buf = PathBuf::from_str(path).unwrap();
    if !path_buf.is_dir() {
        return Ok(None);
    }
    match fs::read_dir(path_buf) {
        Ok(entrys) => {
            let mut list = Vec::new();
            for entry in entrys {
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
        Err(e) => {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("read dir fail :: {} :: {}", path, e),
            })
        }
    }
}

pub fn remove(path: &String) -> Result<(), AliasError> {
    let path_buf = PathBuf::from_str(path).unwrap();
    fs::remove_file(path_buf.display().to_string()).map_err(|e| AliasError {
        err: AliasErrorCode::Unkonw,
        msg: format!("remove file fail :: {} :: {}", path_buf.display(), e),
    })
}

pub fn overwrite(
    file: &mut File,
    path_string: &String,
    content: &String,
) -> Result<(), AliasError> {
    overwrite_with_bytes(file, path_string, content.as_bytes())
}

pub fn overwrite_with_bytes(
    file: &mut File,
    path_string: &String,
    content: &[u8],
) -> Result<(), AliasError> {
    if let Err(e) = file.write_all(content) {
        return Err(AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("rewrite file fail :: {} :: {}", path_string, e),
        });
    }
    Ok(())
}
