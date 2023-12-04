/*
Now I am become shitty lazy code, the destroyer of production.
 */

pub fn day3part1(input: Vec<String>) -> i32 {
    let rows = input.len();
    let cols = input[0].len();
    let mut sum = 0;

    let mut visited = vec![vec![false; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if visited[i][j] || !input[i].chars().nth(j).unwrap().is_digit(10) {
                continue;
            }
            let mut number = 0;
            let mut k = j;
            while k < cols && input[i].chars().nth(k).unwrap().is_digit(10) {
                number = number * 10 + input[i].chars().nth(k).unwrap().to_digit(10).unwrap();
                visited[i][k] = true;
                k += 1;
            }

            if is_adjacent_to_symbol(&input, i, j, k, rows, cols) {
                sum += number as i32;
            }
        }
    }

    sum
}

pub fn day3part2(input: Vec<String>) -> i64 {
    let rows = input.len();
    let cols = input[0].len();
    let mut sum = 0;

    for i in 0..rows {
        for j in 0..cols {
            if input[i].chars().nth(j).unwrap() == '*' {
                let mut part_numbers = vec![];

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let new_i = i as i32 + dx;
                        let new_j = j as i32 + dy;

                        if new_i >= 0 && new_i < rows as i32 && new_j >= 0 && new_j < cols as i32 {
                            let adjacent_char = input[new_i as usize].chars().nth(new_j as usize).unwrap();
                            if adjacent_char.is_digit(10) {
                                let number = extract_number(&input, new_i as usize, new_j as usize);
                                if let Some(num) = number {
                                    if !part_numbers.contains(&num) {
                                        part_numbers.push(num);
                                    }
                                }
                            }
                        }
                    }
                }

                if part_numbers.len() == 2 {
                    sum += part_numbers[0] * part_numbers[1];
                }
            }
        }
    }

    sum
}

fn extract_number(input: &[String], mut i: usize, mut j: usize) -> Option<i64> {

    while j > 0 && input[i].chars().nth(j - 1).unwrap().is_digit(10) {
        j -= 1;
    }
    while i > 0 && input[i - 1].chars().nth(j).unwrap().is_digit(10) {
        i -= 1;
    }

    let mut number = 0;
    while j < input[i].len() && input[i].chars().nth(j).unwrap().is_digit(10) {
        number = number * 10 + input[i].chars().nth(j).unwrap().to_digit(10).unwrap() as i64;
        j += 1;
    }

    if number > 0 {
        Some(number)
    } else {
        None
    }
}



fn is_adjacent_to_symbol(input: &[String], start_row: usize, start_col: usize, end_col: usize, rows: usize, cols: usize) -> bool {
    for i in start_row as i32 - 1..=start_row as i32 + 1 {
        for j in start_col as i32 - 1..=end_col as i32 {
            if i >= 0 && i < rows as i32 && j >= 0 && j < cols as i32 {
                let char = input[i as usize].chars().nth(j as usize).unwrap();
                if char != '.' && !char.is_digit(10) {
                    return true;
                }
            }
        }
    }
    false
}