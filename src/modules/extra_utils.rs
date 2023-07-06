/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the 
/// "Repository" entity
/// from the "git2"
/// crate.
use git2::Repository;

/// Imports the "PathBuf"
/// entity from 
/// Rust's "path" module.
use std::path::PathBuf;

/// Importing the 
/// "Server" entity
/// from the "file-serve"
/// crate.
use file_serve::Server;

/// Importing this crate's error
/// structure.
use super::error::CoutilsError;

/// Serves a directory on the local
/// machine. Returns nothing.
pub fn serve_dir(
    project_path: &String
) -> Result<String, CoutilsError> { 
    let mut result: String = String::from("");  
    let server = file_serve::Server::new(&project_path);
    let serve_op = match server.serve(){
        Ok(_x) => {
            result = server.addr().to_owned();
        },
        Err(e) => {
            return Err::<String, CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    };
    return Ok(result);
}

/// Clones a GitHub repository from the "repo" URL
/// into "target_dir". Returns a boolean to say whether
/// this operation was successful!
pub fn clone_repo(repo: &String, target_dir: &String) -> Result<(), CoutilsError> {
    let repo = match Repository::clone(&repo, target_dir) {
        Ok(_x) => {},
        Err(e) => {
            return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    };
    return Ok(());
}