use crate::input_reader::read_input_file;
use crate::d1::{day1part1, day1part2};
use crate::d2::{day2part1, day2part2};
use crate::d3::{day3part1, day3part2};

mod d1;
mod input_reader;
mod d2;
mod d3;

fn main() {
    let d1_inputs = read_input_file("/home/jamie/git/advent-of-code-2023/inputs/d1.txt");
    let d1p1 = day1part1(d1_inputs.clone());
    let d1p2 = day1part2(d1_inputs);
    println!("DAY 1 PART 1: {0}, PART 2: {1}", d1p1, d1p2);

    let d2_inputs = read_input_file("/home/jamie/git/advent-of-code-2023/inputs/d2.txt");
    let d2p1 = day2part1(d2_inputs.clone());
    let d2p2 = day2part2(d2_inputs);
    println!("DAY 2 PART 1: {0}, PART 2: {1}", d2p1, d2p2);

    let d3_inputs = read_input_file("/home/jamie/git/advent-of-code-2023/inputs/d3.txt");
    let d3p1 = day3part1(d3_inputs.clone());
    let d3p2 = day3part2(d3_inputs);

    println!("DAY 3 PART 1: {0}, PART 2: {1}", d3p1, d3p2);
}


