/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

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

/// Raises to "base" to the power of "power" and
/// returns the result.
pub fn raise_to(base: &u32, power: &u32) -> u32 {
    return base.pow(*power);
}