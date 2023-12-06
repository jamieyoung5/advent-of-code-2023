use crate::input_reader::{read_input_file, read_input_file_raw};
use crate::d1::{day1part1, day1part2};
use crate::d2::{day2part1, day2part2};
use crate::d3::{day3part1, day3part2};
use crate::d4::{day4part1, day4part2};
use crate::d5::{day5part1, day5part2};
use crate::d6::{day6part1, day6part2};

mod d1;
mod input_reader;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod util;

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


    let d4_inputs = read_input_file("/home/jamie/git/advent-of-code-2023/inputs/d4.txt");
    let d4p1 = day4part1(d4_inputs.clone());
    let d4p2 = day4part2(d4_inputs);
    println!("DAY 4 PART 1: {0}, PART 2: {1}", d4p1, d4p2);

    let d5_input = read_input_file_raw("/home/jamie/git/advent-of-code-2023/inputs/d5.txt");
    let d5p1 = day5part1(d5_input.clone());
    let d5p2 = day5part2(d5_input);
    println!("DAY 5 PART 1: {0}, PART 2: {1}", d5p1, d5p2);

    let d6_inputs = read_input_file("/home/jamie/git/advent-of-code-2023/inputs/d6.txt");
    let d6p1 = day6part1(d6_inputs.clone());
    let d6p2 = day6part2(d6_inputs);

    println!("DAY 6 PART 1: {0}, PART 2: {1}", d6p1, d6p2);
}


