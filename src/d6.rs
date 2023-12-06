use crate::util::{big_digits_to_list, digits_to_list};

pub fn day6part1(input: Vec<String>) -> u64 {
    let races: Vec<(u64, u64)> = parse_input(input);
    let mut total = 1;

    for race in races{
        let winning_press_duration = match calculate_button_press_duration(race.0 as f64, race.1 as f64) {
            Some(duration) => duration,
            None => panic!("No feasable answer found for button press duration"),
        };

        let min_duration = winning_press_duration.floor() as u64;
        let max_button_time = race.0 - (min_duration + 1);

        total = total * (max_button_time - min_duration);
    }

    total
}

pub fn day6part2(mut input: Vec<String>) -> u64 {
    for s in input.iter_mut() {
        *s = s.replace(" ", "");
    }

    day6part1(input)
}

/*essentially calculates (t-sqrt(t^2 - 4*d))/2
t is the time of the race
d is the distance you are calculating for
the equation will calculate how long the button was held with the given time and distance
*/
fn calculate_button_press_duration(time: f64, distance: f64) -> Option<f64> {
    let discriminant = time.powi(2) - 4.0 * distance;

    if discriminant < 0.0 {
        // No real solutions
        None
    } else {
        let b = (time - discriminant.sqrt()) / 2.0;

        if b >= 0.0 && b <= time {
            Some(b)
        } else {
            None
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<(u64, u64)>{
    let raw_times: Vec<&str> = input[0].split(":").collect();
    let raw_distances: Vec<&str> = input[1].split(":").collect();
    let times = big_digits_to_list(raw_times[1]);
    let distances = big_digits_to_list(raw_distances[1]);

    times.into_iter().zip(distances.into_iter()).collect()
}

