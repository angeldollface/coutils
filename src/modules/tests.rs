/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "is_int"
/// function from the "coutils"
/// module.
use super::coutils::is_int;

/// Importing the "raise_to"
/// function from the "coutils"
/// module.
use super::coutils::raise_to;

/// Importing the "parse_int"
/// function from the "coutils"
/// module.
use super::coutils::parse_int;

/// Importing the "get_index"
/// function from the "coutils"
/// module.
use super::coutils::get_index;

/// Importing the "has_index"
/// function from the "coutils"
/// module.
use super::coutils::has_index;

/// Importing the "reverse_vec"
/// function from the "coutils"
/// module.
use super::coutils::reverse_vec;

/// Importing the "clean_split"
/// function from the "coutils"
/// module.
use super::coutils::clean_split;

/// Importing the "remove_last"
/// function from the "coutils"
/// module.
use super::coutils::remove_last;

/// Importing the "get_last_item"
/// function from the "coutils"
/// module.
use super::coutils::get_last_item;

/// Testing the "clean_split" function.
#[test]
pub fn test_clean_split() -> () {
    let test_string: String = String::from("Hello World!");
    let split_char: String = String::from(" ");
    let result_vec: Vec<String> = vec![
        String::from("Hello"),
        String::from("World!")
    ];
    assert_eq!(
        clean_split(&test_string, &split_char),
        result_vec
    );
}

/// Testing the "get_index" function.
#[test]
pub fn test_get_index() -> () {
    let index: usize = 1;
    let test_vec: Vec<usize> = vec![1,2,3,4];
    assert_eq!(
        get_index(&test_vec,&2),
        index
    );
}

/// Testing the "has_index" function.
#[test]
pub fn test_has_index() -> () {
    let result: bool = true;
    let test_vec: Vec<usize> = vec![1,2,3,4];
    assert_eq!(
        has_index(&test_vec,&1),
        result
    );
}

/// Testing the "remove_last" function.
#[test]
pub fn test_remove_last() -> () {
    let mut test_vec_full: Vec<usize> = vec![1,2,3,4];
    let test_vec_stripped: Vec<usize> = vec![1,2,3];
    assert_eq!(
        remove_last(&mut test_vec_full),
        test_vec_stripped
    );
}

/// Testing the "is_int" function.
#[test]
pub fn test_is_int() -> () {
    let true_string: String = String::from("1");
    let false_string: String = String::from("A");
    assert_eq!(
        is_int(&true_string),
        true
    );
    assert_eq!(
        is_int(&false_string),
        false
    );
}

/// Testing the "parse_int" function.
#[test]
pub fn test_parse_int() -> () {
    let true_string: String = String::from("1");
    let false_string: String = String::from("A");
    assert_eq!(
        parse_int(&true_string),
        1
    );
    assert_eq!(
        parse_int(&false_string),
        0
    );
}

/// Testing the "reverse_vec" function.
#[test]
pub fn test_reverse_vec() -> () {
    let test_vec_norm: Vec<usize> = vec![1,2,3,4];
    let test_vec_rev: Vec<usize> = vec![4,3,2,1];
    assert_eq!(
        reverse_vec(
            &test_vec_norm
        ),
        test_vec_rev
    );
}

/// Testing the "raise_to" function.
#[test]
pub fn test_raise_to() -> () {
    let base: u32 = 2;
    let power: u32 = 3;
    let result: u32 = 8;
    assert_eq!(
        raise_to(
            &base,
            &power
        ),
        result
    );
}

/// Testing the "get_last_item" function.
#[test]
pub fn test_get_last_item() -> () {
    let test_vec: Vec<usize> = vec![1,2,3,4];
    let last: usize = 4;
    assert_eq!(
        get_last_item(
            &test_vec
        ),
        last
    );
}