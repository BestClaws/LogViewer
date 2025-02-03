use std::io::{BufRead, BufReader};

pub fn from_logfile(path: String) -> impl Iterator<Item = String> {
    let contents = String::new();
    let file = std::fs::File::open(path).unwrap();
    BufReader::new(file).lines().map(Result::unwrap)
}
