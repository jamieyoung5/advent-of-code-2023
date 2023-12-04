use std::collections::{BTreeMap, HashMap};
use regex::{Captures, Regex};

pub fn day1part1(inputs: Vec<String>) -> u32 {
    let valid_digits = [
        "1", "2", "3",
        "4", "5", "6", "7",
        "8", "9",
    ].to_vec();


    let mut total_sum: u32 = 0;

    for input in inputs {
        let calibration_parts = parse_valid_digits_from_input(input, valid_digits.clone());
        let calibration_value = calibration_parts.0 + &calibration_parts.1;

        total_sum += calibration_value.parse::<u32>().unwrap();
    }

    total_sum
}

pub fn day1part2(inputs: Vec<String>) -> u32 {
    let valid_digits = [
        "one", "1", "two", "2", "three", "3",
        "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ].to_vec();

    let mut total_sum: u32 = 0;
    for input in inputs {
        let calibration_parts = parse_valid_digits_from_input(input, valid_digits.clone());

        let first_calibration_part = convert_word_to_digit(calibration_parts.0);
        let last_calibration_part = convert_word_to_digit(calibration_parts.1);

        let calibration_value = first_calibration_part + &last_calibration_part;

        total_sum += calibration_value.parse::<u32>().unwrap();
    }

    total_sum
}

/*
given a string, if that string contains a digit as a word (e.g 'one', 'two', 'three'...) will return
that word as a digit, e.g ('1', '2', '3')
 */
fn convert_word_to_digit(input: String) -> String {
    let digit_map: HashMap<&str, &str> = [
        ("zero", "0"), ("one", "1"), ("two", "2"), ("three", "3"),
        ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"),
        ("eight", "8"), ("nine", "9"),
    ].iter().cloned().collect();

    let re = Regex::new(r"\b(zero|one|two|three|four|five|six|seven|eight|nine)\b").unwrap();
    re.replace_all(&input, |caps: &Captures| {
        digit_map.get(caps.get(0).unwrap().as_str()).unwrap()
    }).to_string()
}

/*
given a vector of all valid keywords, will create a map of those keywords and their appearences in a given string
the map key is an index, the map value is the value

returns the first and last value of the map
*/
fn parse_valid_digits_from_input(input: String, valid_digits: Vec<&str>) -> (String, String) {
    let input_ref = &input;

    let input_digits: BTreeMap<usize, &str> = valid_digits.iter()
        .flat_map(|&word| find_substring_indices(input_ref, word).into_iter().map(move |index| (index, word)))
        .collect();

    let first_digit = input_digits.iter().next().map(|(_, &v)| v);
    let last_digit = input_digits.iter().next_back().map(|(_, &v)| v);

    match(first_digit, last_digit) {
        (Some(first), Some(last)) => {
            (first.to_string(), last.to_string())
        },
        (Some(single), None) | (None, Some(single)) => {
            (single.to_string(), single.to_string())
        },
        (None, None) => {
            panic!("No first or last digits were found in the input");
        }
    }
}

fn find_substring_indices(haystack: &str, needle: &str) -> Vec<usize> {
    haystack.match_indices(needle).map(|(i, _)| i).collect()
}

