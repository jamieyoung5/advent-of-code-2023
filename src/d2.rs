use std::ptr::eq;

struct GameInfo {
    id: u32,
    blue: Vec<u32>,
    green: Vec<u32>,
    red: Vec<u32>
}

pub fn day2part1(inputs: Vec<String>) -> u32 {
    let mut total_sum = 0;

    for input in inputs {
        let game_info_result = parse_game_info(input.clone());
        let game_info = match game_info_result {
            Ok(info) => info,
            Err(error) => panic!("Error while parsing game info: {}", error)
        };

        let red_possible = validate_cube_set(game_info.red, 12);
        let green_possible = validate_cube_set(game_info.green, 13);
        let blue_possible = validate_cube_set(game_info.blue, 14);

        if red_possible && green_possible & blue_possible {
            total_sum += game_info.id;
        }
    }

    total_sum
}

pub fn day2part2(inputs: Vec<String>) -> u32 {
    let mut total_sum = 0;

    for input in inputs {
        let game_info_result = parse_game_info(input.clone());
        let game_info = match game_info_result {
            Ok(info) => info,
            Err(error) => panic!("Error while parsing game info: {}", error)
        };

        let max_red = find_max_cubes(game_info.red);
        let max_green = find_max_cubes(game_info.green);
        let max_blue = find_max_cubes(game_info.blue);

        total_sum += max_red * max_green * max_blue;
    }

    total_sum
}

fn find_max_cubes(cube_set: Vec<u32>) -> u32 {
    let mut max: u32 = 0;
    for cube in cube_set {
        if cube > max {
            max = cube;
        }
    }

    max
}

fn validate_cube_set(cube_set: Vec<u32>, max_cubes: u32) -> bool {
    for cubes in cube_set {
        if cubes > max_cubes {
            return false
        }
    }

    true
}

fn parse_game_info(text: String) -> Result<GameInfo, String> {
    let parts: Vec<&str> = text.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err("Invalid format".to_string());
    }

    let id = parse_id(parts[0])?;
    let (blue, green, red) = parse_colors(parts[1])?;

    Ok(GameInfo { id, blue, green, red })
}

fn parse_id(id_part: &str) -> Result<u32, String> {
    if !id_part.trim().starts_with("Game") {
        return Err("Invalid game ID format".to_string());
    }
    id_part["Game".len()..].trim().parse::<u32>()
        .map_err(|_| "Invalid game ID".to_string())
}

fn parse_colors(colors_part: &str) -> Result<(Vec<u32>, Vec<u32>, Vec<u32>), String> {
    let mut blue = Vec::new();
    let mut green = Vec::new();
    let mut red = Vec::new();

    for color_count_pair in colors_part.split(';') {
        for color_count in color_count_pair.split(',') {
            let pair = color_count.trim().split_whitespace().collect::<Vec<&str>>();
            if pair.len() != 2 {
                return Err(format!("Invalid color format in part '{}'", color_count));
            }

            let count = pair[0].parse::<u32>().map_err(|_| format!("Invalid count in part '{}'", pair[0]))?;
            match pair[1].to_lowercase().as_str() {
                "blue" => blue.push(count),
                "green" => green.push(count),
                "red" => red.push(count),
                _ => return Err(format!("Unknown color '{}'", pair[1])),
            }
        }
    }

    Ok((blue, green, red))
}
