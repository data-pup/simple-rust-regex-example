extern crate regex;

use std::str::FromStr;

static ITEMS: &[&'static str] = &["why", "hello", "hell", "world", "how", "are", "you"];

/// Simple testing function, imagine that the program's arguments are
/// ["hell*", "[a-z]{3}"].
fn get_args() -> Vec<String> {
    let res: Result<Vec<String>, _> = ["hell*", "^[a-z]{5}$"]
        .into_iter()
        .map(|s| String::from_str(s))
        .collect();
    res.unwrap()
}

fn init_regexps(args: &[String]) -> Vec<regex::Regex> {
    args.iter()
        .map(|s| regex::Regex::from_str(s).unwrap())
        .collect()
}

fn main() {
    let args = get_args();
    let regexps = init_regexps(&args);
    for item in ITEMS.iter() {
        let match_exists: bool = regexps
            .iter()
            .map(|r| r.is_match(&item))
            .fold(false, |res, curr| res || curr);
        if match_exists {
            println!("{}", item);
        }
    }
}
