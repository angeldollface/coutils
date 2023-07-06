/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Using Rust's standard
/// "Path" API from the
/// "path" module.
use std::path::Path;

/// Importing the "PartialEq"
/// trait from Rust's "cmp"
/// module.
use std::cmp::PartialEq;

/// Importing the method
/// to copy directories
/// from the "fs-extra"
/// crate.
use fs_extra::dir::copy;

/// Importing the "create_dir"
/// function from Rust's
/// "fs" module.
use std::fs::create_dir;

/// Importing the "read_dir"
/// function to list the contents 
/// of a directory.
use std::fs::read_dir;

/// Importing Rust's "remove_dir_all"
/// function from the "fs" module
/// to remove directories.
use std::fs::remove_dir_all;

/// We import the "move_dir"
/// method from the "fs_extra"
/// crate.
use fs_extra::dir::move_dir;

/// Importing this crate's error
/// structure.
use super::error::CoutilsError;

/// Importing the "Entity" enum
/// from this crate's "file_utils"
/// module because "FileEntry"
/// needs it.
use super::file_utils::Entity;

/// We need this entity to
/// perform copying operations
/// on directories.
use fs_extra::dir::CopyOptions;

/// Tries to copy a folder from "src" to "target"
/// and returns a boolean depending on whether the
/// operation succeeded or not.
pub fn folder_copy(src: &String, target: &String) -> Result<(), CoutilsError> {
    let options = CopyOptions::new();
    let copy_op = copy(src, target, &options);
    match copy_op {
        Ok(_n) => {},
        Err(e) => {
            return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    }
    return Ok(());
}

/// Attempts to move a directory from "src" to "target".
/// A boolean is returned depending on whether the operation
/// suceeded or not.
pub fn dir_move(src: &String, target: &String) ->  Result<(), CoutilsError> {
    let options = CopyOptions::new();
    let move_op = move_dir(src, target, &options);
    match move_op {
        Ok(_n) => {},
        Err(e) => {
            return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    }
    return Ok(());
}

/// Tries to create a new directory and returns
/// a boolean depending on whether the
/// operation succeeded or not.
pub fn create_directory(path: &String) ->  Result<(), CoutilsError> {
    let new_dir = create_dir(path);
    match new_dir {
        Ok(_n) => {},
        Err(e) => {
            return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    }
    return Ok(());
}

/// Checks whether a directory exists.
pub fn dir_is(dir: &String) -> bool {
    return Path::new(dir).is_dir();
}

/// A data structure to represent
/// a file entry in a file system.
#[derive(PartialEq, Clone)]
pub struct FileEntry {
    pub name: String,
    pub file_type: Entity
}

/// Implementing methods
/// for the "FileEntry"
/// entity.
impl FileEntry {

    /// Convenience method
    /// to create a new instance
    /// of the "FileEntry" entity.
    pub fn new(name: &String, file_type: &Entity) -> FileEntry {
        return FileEntry { 
            name: name.to_owned(), 
            file_type: file_type.to_owned() 
        }
    }
}

/// A method to return the contents of a directory.
/// Returns this information in the form of a vector of the
/// "FileEntry" entity. Skips all invalid or non-existent entries.
pub fn list_dir_contents(dir: &str) -> Result<Vec<FileEntry>, CoutilsError> {
    let mut result: Vec<FileEntry> = Vec::new();
    let mut dirs = match read_dir(dir) {
        Ok(dirs) => dirs,
        Err(e) => {
            return Err::<Vec<FileEntry>, CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    };
    for item in dirs {
        match item {
            Ok(dir_item) => {
                let path_item: &String = &dir_item.path().display().to_string();
                if Path::new(path_item).is_dir() {
                    result.push(
                        FileEntry::new(
                            path_item,
                            &Entity::Dir
                        )
                    );
                }
                else {
                    result.push(
                        FileEntry::new(
                            &path_item,
                            &Entity::File
                        )
                    );
                }
            },
            Err(e) => {
                return Err::<Vec<FileEntry>, CoutilsError>(CoutilsError::new(&e.to_string()));
            }
        };
    }
    return Ok(result);
}

/// Deletes a directory and returns 
/// a boolean depending on whether 
/// the operation succeeded or not.
pub fn del_dir(path: &str) -> Result<(), CoutilsError> {
    let del_op = remove_dir_all(path);
    match del_op {
        Ok(_x) => {},
        Err(e) => {
            return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    };
    return Ok(());
}