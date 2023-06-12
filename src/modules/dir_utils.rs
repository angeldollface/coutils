/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Using Rust's standard
/// "Path" API from the
/// "path" module.
use std::path::Path;

/// Rust's file metadata
/// API from the "fs"
/// module.
use std::fs::metadata;

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
pub fn folder_copy(src: String, target: String) -> bool {
    let mut result: bool = false;
    let options = CopyOptions::new();
    let copy_op = copy(src, target, &options);
    match copy_op {
        Ok(_n) => result = true,
        Err(_x) => {}
    }
    return result;
}

/// Attempts to move a directory from "src" to "target".
/// A boolean is returned depending on whether the operation
/// suceeded or not.
pub fn dir_move(src: String, target: String) -> bool {
    let mut result: bool = false;
    let options = CopyOptions::new();
    let move_op = move_dir(src, target, &options);
    match move_op {
        Ok(_n) => result = true,
        Err(_x) => {}
    }
    return result;
}

/// Tries to create a new directory and returns
/// a boolean depending on whether the
/// operation succeeded or not.
pub fn create_directory(path: String) -> bool {
    let mut result: bool = false;
    let new_dir = create_dir(path);
    match new_dir {
        Ok(_n) => result = true,
        Err(_x) => {}
    }
    return result;
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
pub fn list_dir_contents(dir: &str) -> Vec<FileEntry> {
    let mut result: Vec<FileEntry> = Vec::new();
    for item in read_dir(dir).unwrap() {
        match item {
            Ok(dir_item) => {
                let path_item: &String = &dir_item.path().display().to_string();
                if metadata(path_item).unwrap().is_dir() {
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
                // Do nothing.
            }
        };
    }
    return result;
}

/// Deletes a directory and returns 
/// a boolean depending on whether 
/// the operation succeeded or not.
pub fn del_dir(path: &str) -> bool {
    let mut result: bool = false;
    let del_op = remove_dir_all(path);
    match del_op {
        Ok(_x) => result = true,
        Err(_e) => {}
    };
    return result;
}