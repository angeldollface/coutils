/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing Rust's 
/// "File" struct.
use std::fs::File;

/// Importing Rust's "write"
/// function.
use std::fs::write;

/// Rust's file metadata
/// API.
use std::fs::metadata;

/// Importing the "PartialEq"
/// trait from Rust's "cmp"
/// module.
use std::cmp::PartialEq;

/// Importing Rust's
/// "read_to_string" function.
use std::fs::read_to_string;

/// We import the "move_file"
/// method from the "fs_extra"
/// crate.
use fs_extra::file::move_file;

/// We need this structure to
/// perform copying operations
/// on files.
use fs_extra::file::CopyOptions;

/// An enum to list
/// the file types.
#[derive(PartialEq, Clone)]
pub enum Entity{
    Dir,
    File,
    Unknown
}

/// Tries to move a file from "src" to "target"
/// and returns a boolean depending on whether the
/// operation succeeded.
pub fn file_move(src: String, target: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let options = CopyOptions::new();
    let move_op = move_file(src, target, &options);
    match move_op {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Checks whether a file exists and
/// returns a boolean to that effect.
pub fn file_is(filename: &String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let contents = read_to_string(filename);
    match contents {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

// Tries to create a file and returns
/// a boolean depending on whether the
/// operation succeeded.
pub fn create_file(filename: &String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let new_file = File::create(filename);
    match new_file {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to write to a file and returns
/// a boolean depending on whether the
/// operation succeeded.
pub fn write_to_file(
    filename: &String, 
    contents: &String
) -> bool {
    let mut result: Vec<bool> = Vec::new();
    if file_is(filename) == true {
        let write_op = write(filename, contents);
        match write_op {
            Ok(_n) => result.push(true),
            Err(_x) => result.push(false)
        }
    }
    return result[0];
}

/// Tries to read a file and return
/// its contents as a contents.
pub fn read_file(filename: &String) -> String {
    let mut result: String = String::from("");
    if file_is(filename) == true {
        result = read_to_string(filename).unwrap();
    }
    else {}
    return result;
}

/// Checks whether "entity" is a directory or
/// file.
pub fn file_type(entity: &String) -> Entity {
    let mut result: Entity = Entity::Unknown;
    if metadata(entity).unwrap().is_dir() {
        result = Entity::Dir;
    }
    else if metadata(entity).unwrap().is_file(){
        result = Entity::File;
    }
    else {
        result = Entity::Unknown;
    }
    return result;
}