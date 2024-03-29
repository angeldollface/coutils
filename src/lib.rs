/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// folder as a module.
pub mod modules;

/// Re-exporting this
/// crate's time
/// module.
pub use modules::time::*;

/// Declaring the "tests"
/// module as the test module
/// and exporting it.
#[cfg(test)]
pub use modules::tests::*;

/// Re-exporting this
/// crate's error-handling
/// module.
pub use modules::error::*;

/// Declaring the module for
/// working with integers
/// and exporting it.
pub use modules::int_utils::*;

/// Declaring the module for
/// working with directories
/// and exporting it.
pub use modules::dir_utils::*;

/// Declaring the module for
/// working with vectors
/// and exporting it.
pub use modules::vec_utils::*;

/// Declaring the module for
/// working with files
/// and exporting it.
pub use modules::file_utils::*;

/// Declaring the module for
/// working with strings
/// and exporting it.
pub use modules::string_utils::*;
