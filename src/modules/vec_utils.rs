/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the
/// range API
/// from the "rand"
/// crate.
use rand::Rng;

/// Importing the "Debug"
/// trait from Rust's "fmt"
/// module.
use std::fmt::Debug;

/// Importing this crate's error
/// structure.
use super::error::CoutilsError;

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
/// Returns a boolean depending on whether the item exists or not.
pub fn has_item<T: Clone + PartialEq>(
    subject: &Vec<T>, 
    item: &T
) -> bool {
    let mut result: bool = false;
    let match_op = &subject.iter().position(|r| r == item);
    match match_op {
        Some(_x) => {
            result = true;
        }
        None => {}
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
/// Returns this item.
pub fn get_rand_item<T: Debug + Clone + PartialEq>(
    subject: &Vec<T>
) -> T {
    let mut range = rand::thread_rng();
    return subject[range.gen_range(0..subject.len())].clone();
}

/// Checks if "subject" has the index "index".
/// Returns a boolean to that effect.
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

/// Attempts to get the index of "item" in "subject" as an unsigned integer.
pub fn get_index<T: Debug + Clone + PartialEq>(
    subject: &Vec<T>, 
    item: &T
) -> Result<usize, CoutilsError> {
    let mut result: usize = match subject.iter().position(|s| s == item){
        Some(result) => result,
        None => {
            let e: String = format!("Element \"{:?}\" does not exist.", &item);
            return Err::<usize, CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    };
    return Ok(result);
}