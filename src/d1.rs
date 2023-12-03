use regex::Regex;

pub fn day1part1(inputs: Vec<String>) -> u32 {
    let regex = Regex::new(r"([0-9]+)");
    let numbers_regex = match regex {
        Ok(numbers) => numbers,
        Err(error) => panic!("Error while parsing regex statement: {:?}", error),
    };
    let mut total_sum: u32 = 0;

    for input in inputs {
        let mut numbers_only = String::new();

        for cap in numbers_regex.captures_iter(&*input) {
            if let Some(matched) = cap.get(1) {
                numbers_only.push_str(matched.as_str());
            }
        }

        let first_number = numbers_only.chars().next().unwrap().to_digit(10).unwrap();
        let last_number = numbers_only.chars().last().unwrap().to_digit(10).unwrap();
        let calibration_value = first_number.to_string() + &last_number.to_string();

        total_sum += calibration_value.parse::<u32>().unwrap();
    }

    return total_sum;
}

pub fn day1part2(mut inputs: Vec<String>) -> u32 {
    for item in &mut inputs {
        *item = replace_number_words(item);
    }

    return day1part1(inputs);
}

fn replace_number_words(input: &str) -> String {
    let number_words = [
        ("one", "1"), ("two", "2"), ("three", "3"),
        ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"),
        ("eight", "8"), ("nine", "9"),
    ];

    let mut found_positions: Vec<(usize, &str)> = Vec::new();

    for (word, numeral) in number_words.iter() {
        if let Some(index) = input.find(word) {
            found_positions.push((index, *numeral));
        }

        if let Some(index) = input.find(numeral) {
            found_positions.push((index, *numeral));
        }

    }

    found_positions.sort_by_key(|&(index, _)| index);

    return found_positions.iter().map(|&(_, value)| value).collect();
}
