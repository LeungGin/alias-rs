use crate::core::error::{AliasError, AliasErrorCode};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub fn create_if_absent(path: &PathBuf) -> Result<File, AliasError> {
    if path.exists() {
        return File::open(&path).map_err(|e| AliasError {
            err: AliasErrorCode::Unkonw,
            msg: format!("open file fail :: {} :: {}", path.display(), e),
        });
    }
    create(path)
}

pub fn create(path: &PathBuf) -> Result<File, AliasError> {
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(AliasError {
                err: AliasErrorCode::Unkonw,
                msg: format!("create dir fail :: {} :: {}", parent.display(), e),
            });
        }
    }
    return File::create(&path).map_err(|e| AliasError {
        err: AliasErrorCode::Unkonw,
        msg: format!("create file fail :: {} :: {}", path.display(), e),
    });
}

pub fn remove(path: &PathBuf) -> Result<(), AliasError> {
    fs::remove_file(path.display().to_string()).map_err(|e| AliasError {
        err: AliasErrorCode::Unkonw,
        msg: format!("remove file fail :: {} :: {}", path.display(), e),
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
