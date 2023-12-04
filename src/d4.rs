use std::collections::HashSet;

pub fn day4part1(inputs: Vec<String>) -> u32 {
    let mut total_sum = 0;

    for input in inputs {
        let (winning_numbers, card_numbers) = parse_card(input);

        let matches: u32 = count_matches(&winning_numbers, &card_numbers);
        if matches > 0 {
            total_sum += 2_u32.pow(matches - 1);
        }
    }

    total_sum
}

pub fn day4part2(inputs: Vec<String>) -> u32 {
    let mut cards = Vec::new();
    for input in &inputs {
        let (winning_numbers, card_numbers) = parse_card(input.clone());
        cards.push((winning_numbers, card_numbers));
    }

    let (total_cards, additional_wins) = process_cards(cards.clone());

    total_cards + process_additional_copies(cards, additional_wins)
}

fn process_additional_copies(cards: Vec<(HashSet<u32>, Vec<u32>)>, mut additional_wins: Vec<u32>) -> u32 {
    let mut total_cards = 0;

    for i in 0..cards.len() {
        while additional_wins[i] > 0 {
            let extra_copies = additional_wins[i];
            total_cards += extra_copies;
            additional_wins[i] = 0;

            let matches = count_matches(&cards[i].0, &cards[i].1) as usize;
            if matches > 0 {
                let max_index = std::cmp::min(i + 1 + matches, cards.len());
                for next_index in i + 1..max_index {
                    additional_wins[next_index] += extra_copies;
                }
            }
        }
    }

    total_cards
}

fn process_cards(cards: Vec<(HashSet<u32>, Vec<u32>)>) -> (u32, Vec<u32>) {
    let mut total_cards = 0;
    let mut additional_wins = vec![0; cards.len()];

    for (current_index, card) in cards.iter().enumerate() {
        let matches = count_matches(&card.0, &card.1) as usize;
        total_cards += 1;

        if matches > 0 {
            let max_index = std::cmp::min(current_index + 1 + matches, cards.len());
            for next_index in current_index + 1..max_index {
                additional_wins[next_index] += 1;
            }
        }
    }

    (total_cards, additional_wins)
}


/*
given all the winning numbers and the card numbers, will count which card numbers are winners, e.g
count how many numbers are shared between the two vectors.
 */
fn count_matches(winning_numbers: &HashSet<u32>, card_numbers: &Vec<u32>) -> u32 {
    let mut count = 0;
    for card_number in card_numbers {
        if winning_numbers.contains(card_number) {
            count += 1;
        }
    }

    count
}


/*
given a line from the inputs of a cards data, it will parse the data of this card into a hashset and a vector
where the winning numbers are the hashset, and the card numbers are the vector
 */
fn parse_card(card: String) -> (HashSet<u32>, Vec<u32>) {
    let parts: Vec<&str> = card.split('|').collect();

    let winning_numbers: HashSet<u32> = parts[0]
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let card_numbers: Vec<u32> = parts[1]
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    (winning_numbers, card_numbers)
}