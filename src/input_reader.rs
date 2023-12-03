use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read_input_file(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("No file found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Unable to parse line"))
        .collect()

}