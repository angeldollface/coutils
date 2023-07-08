/*
COUTILS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Using the "Local"
/// structure from the "chrono"
/// crate to retrieve the current 
/// time.
use chrono::offset::Local;

/// Gets the current time 
/// in the format "YYYY-MM-DD-HH:MM:SS".
pub fn get_time() -> String {
    let time = Local::now();
    let date = time.date_naive();
    let curr_time = time.time();
    let formatted: String = format!(
        "{}-{}",
        date.format("%Y-%m-%d"),
        curr_time.format("%H:%M:%S")
    );
    return formatted;
}