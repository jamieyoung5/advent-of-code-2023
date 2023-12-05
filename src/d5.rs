#[derive(Debug)]
struct ConversionData {
    destination_start: u64,
    source_start: u64,
    range_length: u64
}

pub fn day5part1(input: String) -> u64 {
    let (seed_list, conversion_maps) = parse_input(input);
    let mut lowest_location = u64::MAX;

    for seed in seed_list {
        lowest_location = calculate_seed_lowest_location(seed, &conversion_maps, lowest_location);
    }

    lowest_location
}

pub fn day5part2(input: String) -> u64 {
    let (seed_list, conversion_maps) = parse_input(input);
    let seeds: Vec<_> = seed_list
        .chunks(2)
        .map(|chunk| (chunk[0], chunk.get(1).cloned().unwrap_or_default()))
        .collect();

    let mut lowest_location = u64::MAX;
    for seed_range in seeds {
        for seed in seed_range.0..=(seed_range.0+seed_range.1) {
            lowest_location = calculate_seed_lowest_location(seed, &conversion_maps, lowest_location);
        }
    }

    lowest_location
}

fn calculate_seed_lowest_location(seed: u64, conversion_maps:  &Vec<Vec<ConversionData>>, lowest_location: u64) -> u64 {
    let mut location: u64 = seed;
    for conversion in conversion_maps {
        location = map_value(location, conversion);
    }

    lowest_location.min(location)
}

fn map_value(value: u64, mappings: &Vec<ConversionData>) -> u64 {
    for map in mappings {
        if value >= map.source_start && value < map.source_start + map.range_length {
            return map.destination_start + ( value - map.source_start );
        }
    }

    value
}

fn parse_input(input: String) -> (Vec<u64>, Vec<Vec<ConversionData>>) {
    let split_input: Vec<&str> = input.split("\n\n").collect();

    let seed_list = parse_input_to_seed_list(split_input[0]);

    let mut conversion_maps: Vec<Vec<ConversionData>> = Vec::with_capacity(7);
    for map in split_input.iter().skip(1) {
        let parsed_map = parse_input_conversion_map(map);
        conversion_maps.push(parsed_map);
    }

    (seed_list, conversion_maps)
}

fn parse_input_conversion_map(input: &str) -> Vec<ConversionData> {
    let mut conversion_map_list: Vec<ConversionData> = Vec::new();
    for line in input.lines().skip(1) {
        let map_data = digits_to_list(line);
        let conversion_data = ConversionData {
            destination_start: map_data[0],
            source_start: map_data[1],
            range_length: map_data[2],
        };

        conversion_map_list.push(conversion_data);
    }

    conversion_map_list
}

fn parse_input_to_seed_list(seed_list: &str) -> Vec<u64> {
    let parts: Vec<&str> = seed_list.split(":").collect();

    digits_to_list(parts[1])
}

fn digits_to_list(digits: &str) -> Vec<u64> {
    digits
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect()
}