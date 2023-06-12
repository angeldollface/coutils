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
/// data structure from 
/// Rust's "path" module.
use std::path::PathBuf;

/// Importing the 
/// "Server" entity
/// from the "file-serve"
/// crate.
use file_serve::Server;

/// A data structure to
/// store configuration
/// options for starting a local
/// server instance.
pub struct ServerInfo {

    /// Will our server have
    /// a message while it serves?
    pub has_message: bool,

    /// If so, what will the message be?
    pub server_message: Option<String>
}

/// Implementing methdos
/// for the "ServerInfo"
/// structure.
impl ServerInfo {

    /// Convenience method
    /// to create a new instance
    /// of the "ServeInfo" data structure.
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

/// Serves a directory on the local
/// machine. Returns nothing.
pub fn serve_dir(
    project_path: &String, 
    server_info: &ServerInfo
) -> () {
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

/// Clones a GitHub repository from the "repo" URL
/// into "target_dir". Returns a boolean to say whether
/// this operation was successful!
pub fn clone_repo(repo: String, target_dir: String) -> bool {
    let mut result: bool = false;
    let repo = match Repository::clone(&repo, target_dir) {
        Ok(_x) => {
            result = true;
        },
        Err(_e) => {}
    };
    return result;
}