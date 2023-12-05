use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    fs,
    io::{self, Read},
};

pub fn read_input_file(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("No file found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Unable to parse line"))
        .collect()

}

pub fn read_input_file_raw(path: impl AsRef<Path>) -> String {
    let file_contents = fs::read_to_string(path);
    
    match file_contents {
        Ok(contents) => contents,
        Err(..) => panic!("Failed to read file")
    }
}