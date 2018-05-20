extern crate regex;

use std::str::FromStr;

struct Item {
    name: &'static str,
    size: u32,
}

impl Item {
    pub fn size(&self) -> u32 {
        self.size
    }
}

struct Row {
    regexp: regex::Regex,
    size: u32,
}

static ITEMS: &[Item] = &[
    Item { name: "why", size: 3 },
    Item { name: "hello", size: 5 },
    Item { name: "hell", size: 4 },
    Item { name: "world", size: 5 },
    Item { name: "how", size: 3 },
    Item { name: "are", size: 3 },
    Item { name: "you", size: 3 },
];

/// Simple testing function, imagine that the program's arguments are
/// ["why", "hell*", "[a-z]{3}"].
fn get_args() -> Vec<String> {
    let res: Result<Vec<String>, _> = [
        "why",
        "hell*",
        "[a-z]{3}",
    ].into_iter()
        .map(|s| String::from_str(s))
        .collect();
    res.unwrap()
}

fn init_rows(args: &[String]) -> Vec<Row> {
    args.iter()
        .map(|s| regex::Regex::from_str(s).unwrap())
        .map(|r| Row { regexp: r, size: 0} )
        .collect()
}

fn main() {
    let args = get_args();
    let mut rows = init_rows(&args);
    for item in ITEMS.iter() {
        rows.iter()
            .for_each(|row| {
                if row.regexp.is_match(item.name) {
                    row.size += item.size;
                }
            })
    }

    // ------------------------------------------------------------------
    // Create a regex using a string, note that this is a result.
    // let r = regex::Regex::from_str("hell*").unwrap();

    // Declare an array of items that we will search through for matches.
    // let data = &["why", "hello", "hell", "world"];

    // Iterate through the array, print any items that match the regex.
    // data.iter()
    //     .filter(|item| r.is_match(item))
    //     .for_each(|item| println!("{}", item));
    // ------------------------------------------------------------------
}
