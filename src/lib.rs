/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// folder as a module.
mod modules;

/// Declaring the "tests"
/// module as the test module
/// and exporting it.
#[cfg(test)]
pub use modules::tests;

/// Declaring the main
/// module and exporting it.
pub use modules::coutils;