pub fn digits_to_list(digits: &str) -> Vec<u32> {
    digits
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect()
}

pub fn big_digits_to_list(digits: &str) -> Vec<u64> {
    digits
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect()
}
