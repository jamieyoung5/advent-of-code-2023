use crate::input_reader::read_input_file;
use crate::d1::{day1part2};

mod d1;
mod input_reader;

fn main() {
    let inputs = read_input_file("C:\\Users\\Jamie.Young\\Documents\\Github\\advent-of-code-2023\\inputs\\q1.txt");
    let number = day1part2(inputs);

    println!("{}", number)
}
