/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

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