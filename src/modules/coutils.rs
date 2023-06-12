/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the
/// range API
/// from the "rand"
/// crate.
use rand::Rng;

use git2::Repository;

use std::path::PathBuf;

use file_serve::Server;

use std::fs::create_dir;
use fs_extra::dir::copy;

use fs_extra::dir::move_dir;

use std::fs::remove_dir_all;

use fs_extra::file::move_file;

use fs_extra::file::CopyOptions;

/// Importing Rust's 
/// "File" struct.
use std::fs::File;

use std::fs::read;
/// Importing the "read_dir"
/// function to list the contents 
/// of a directory.
use std::fs::read_dir;

/// Importing Rust's "write"
/// function.
use std::fs::write;

use std::path;
/// Using Rust's standard
/// "Path" API.
use std::path::Path;

/// Importing an extra 
/// standard trait.
use std::fmt::Debug;

/// Rust's file metadata
/// API.
use std::fs::metadata;

/// Importing the "PartialEq"
/// trait.
use std::cmp::PartialEq;

/// Importing Rust's
/// "read_to_string" function.
use std::fs::read_to_string;

/// An enum to list
/// the file types.
#[derive(PartialEq, Clone)]
pub enum Entity{
    Dir,
    File,
    Unknown
}

#[derive(PartialEq, Clone)]
pub struct FileEntry {
    pub name: String,
    pub file_type: Entity
}

impl FileEntry {
     pub fn new(name: &String, file_type: &Entity) -> FileEntry {
        return FileEntry { name: name.to_owned(), file_type: file_type.to_owned() }
     }
}

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

/// Clones a GitHub repository from "repo" into "target_dir".
pub fn clone_repo(repo: String, target_dir: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let repo = match Repository::clone(&repo, target_dir) {
        Ok(_x) => result.push(true),
        Err(_e) => result.push(false)
    };
    return result[0];
}

pub struct ServerInfo {
    pub has_message: bool,
    pub server_message: Option<String>
}

impl ServerInfo {
    pub fn new(
        has_message: &bool,
        server_message: &Option<String>
    ) -> ServerInfo {
        return ServerInfo{
            has_message: has_message.to_owned(),
            server_message: server_message.to_owned()
        }
    }
}

/// Serves the "build" dir
/// on this address: https://localhost:1024.
pub fn serve_dir(
    project_path: &String, 
    server_info: &ServerInfo
) {
    let mut path: PathBuf = PathBuf::new();
    path.push(project_path);
    let server_instance: Server = Server::new(path);
    if server_info.has_message {
        match &server_info.server_message {
            Some(msg) => {
                println!("{}", msg);
            },
            None => {
                // Do nothing.
            }
        }
    }
    else {
        // Do nothing.
    }
    server_instance.serve().unwrap_or_else(
        |error| {
            println!("{}", error);
        }
    );
}

/// Tries to copy a folder from "src" to "target"
/// and returns a boolean depending on whether the
/// operation succeeded.
pub fn folder_copy(src: String, target: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let options = fs_extra::dir::CopyOptions::new();
    let copy_op = copy(src, target, &options);
    match copy_op {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Attempts to move a directory from "src" to "target".
/// A boolean is returned depending on whether the operation
/// suceeded.
pub fn dir_move(src: String, target: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let options = fs_extra::dir::CopyOptions::new();
    let move_op = move_dir(src, target, &options);
    match move_op {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to create a new directory and returns
/// a boolean depending on whether the
/// operation succeeded.
pub fn create_directory(path: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let new_dir = create_dir(path);
    match new_dir {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
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

// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(
    subject: &String, 
    split_char: &String
) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&*split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
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

/// Attempts to get the index of "item" in "subject" as an unsigned integer.
pub fn get_index<T: Debug + Clone + PartialEq>(
    subject: &Vec<T>, 
    item: &T
) -> usize {
    return subject.iter().position(|s| s == item).unwrap();
}

/// Checks whether a directory exists.
pub fn dir_is(dir: &String) -> bool {
    let result: bool = Path::new(dir).is_dir();
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

/// Checks if "subject" has the index "index".
pub fn has_index<T: Debug + Clone>(
    subject: &Vec<T>, 
    index: &usize
) -> bool {
    let mut result: bool = false;
    if index >= &subject.len(){}
    else {
        result = true;
    }
    return result;
}

// Removes the last item of a vector with integers in
// them ("usize").
pub fn remove_last<T: Debug + Clone + PartialEq>(
    subject: &mut Vec<T>
) -> Vec<T> {
    let vec_last_index: usize = &subject.len() - 1;
    subject.remove(vec_last_index);
    return subject.to_vec();
}

/// Checks whether the supplied character is
/// an integer. Returns a boolean depending on
/// whether this is the case.
pub fn is_int(subject: &String) -> bool{
    let mut result: bool = false;
    let match_op = subject.parse::<usize>();
    match match_op {
        Ok(_n) => {
            result = true
        },
        Err(_e) => {
            // Do nothing.
        }
    };
    return result;
}

/// We check if "subject" is an integer and return
/// it as such if it is. If not, 0 is returned.
pub fn parse_int(subject: &String) -> usize {
    let mut result: usize = 0;
    if is_int(&subject) {
        result = subject.parse::<usize>().unwrap();
    }
    else {
        // Do nothing.
    }
    return result;
}

/// Gets the last item of a string array and returns it.
pub fn get_last_item<T: Debug + Clone + PartialEq>(
    arr: &Vec<T>
) -> T {
    let array_length: usize = arr.len();
    let last_item_index: usize = array_length -1;
    return arr[last_item_index].clone();
}

/// Get a random item from a string vector.
pub fn get_rand_item<T: Debug + Clone + PartialEq>(
    subject: &Vec<T>
) -> T {
    let mut range = rand::thread_rng();
    return subject[range.gen_range(0..subject.len())].clone();
}

/// Raises to "base" to the power of "power" and
/// returns the result.
pub fn raise_to(base: &u32, power: &u32) -> u32 {
    return base.pow(*power);
}

/// Reverses the order of a vector and 
/// returns the reversed vector.
pub fn reverse_vec<T: Clone>(subject: &Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let last_index: usize = subject.len();
    for i in (0..last_index).rev() {
        result.push(subject[i].clone());
    }
    return result;
}

/// Checks whether an item exists in a vector. Any type allowed.
pub fn has_item<T: Clone + PartialEq>(subject: &Vec<T>, item: &T) -> bool {
    let mut result: bool = false;
    let match_op = &subject.iter().position(|r| r == item);
    match match_op {
        Some(_x) => {
            result = true;
        }
        None => {
            // Do nothing.
        }
    }
    return result;
}