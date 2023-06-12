use crate::common::error;
use crate::common::error::Result;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn data_dir() -> Result<PathBuf> {
    let path = Path::new("fluent/data");
    let home_dir = home::home_dir().ok_or(error::proto_parse_error_with_str("home dir is none"))?;
    let data_dir = home_dir.join(path);
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }
    return Ok(data_dir);
}
// pub fn home_dir() -> Result<PathBuf> {
//     let home_path =
//         home::home_dir().ok_or(error::proto_parse_error_with_str("home dir is none"))?;
//     if !home_path.exists() {
//         fs::create_dir_all(&home_path)?
//     }
//     return Ok(home_path);
// }

pub fn copy_proto_file(to_path: &PathBuf, file: &PathBuf) -> Result {
    if file.is_dir() {
        let a = fs::read_dir(file)?;
        let p = end_path(file)?;
        let to_path = to_path.join(p);
        for f in a {
            let a = f?.path();
            copy_proto_file(&to_path, &a)?;
        }
    } else {
        let file_name = file
            .file_name()
            .ok_or(error::proto_parse_error_with_str(
                "copy_proto_file file_name osStr to string error",
            ))?
            .to_str()
            .ok_or(error::proto_parse_error_with_str(
                "copy_proto_file file_name osStr to string error",
            ))?;
        if !file_name.ends_with("proto") {
            return Ok(());
        }

        if !to_path.exists() {
            fs::create_dir_all(to_path)?;
        }

        let to_path = to_path.join(file_name);
        fs::copy(file, &to_path)?;
    }
    return Ok(());
}

pub fn delete_proto_file(file: &PathBuf) -> Result {
    if file.is_dir() {
        let dir = fs::read_dir(file)?;
        for d in dir {
            let f = d?.path();
            delete_proto_file(&f)?;
        }
        fs::remove_dir(file)?;
    } else {
        fs::remove_file(file)?;
    }
    return Ok(());
}

pub fn end_path(path: &PathBuf) -> Result<String> {
    let vec: Vec<&OsStr> = path.iter().collect();
    let p = vec
        .get(vec.len() - 1)
        .ok_or(error::proto_parse_error_with_str(
            "end_path get end path is none",
        ))?
        .to_str()
        .ok_or(error::proto_parse_error_with_str(
            "end_path get end path is none",
        ))?
        .to_string();
    return Ok(p);
}
