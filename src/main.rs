extern crate regex;

use std::str::FromStr;

fn main() {
    // Create a regex using a string, note that this is a result.
    let r = regex::Regex::from_str("hell*").unwrap();

    // Declare an array of items that we will search through for matches.
    let data = &["why", "hello", "hell", "world"];

    // Iterate through the array, print any items that match the regex.
    data.iter()
        .filter(|item| r.is_match(item))
        .for_each(|item| println!("{}", item));
}
