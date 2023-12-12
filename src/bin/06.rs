use regex::Regex;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_races(input);

    let product_of_possible_wins: u64 = races.iter().map(get_amount_of_wins).product();

    Some(product_of_possible_wins)
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_race(input);

    Some(get_amount_of_wins(&race))
}

fn parse_races(input: &str) -> Vec<(u64, u64)> {
    let space_regex = Regex::new(r"\s+").unwrap();
    let mut lines = input.lines();
    let times_string = lines.next().unwrap().split(':').nth(1).unwrap().trim();
    let distances_string = lines.next().unwrap().split(':').nth(1).unwrap().trim();

    let times_split = space_regex.split(times_string);
    let distances_split = space_regex.split(distances_string);

    times_split
        .zip(distances_split)
        .map(|(time, distance)| {
            let time = time.parse::<u64>().unwrap();
            let record_distance = distance.parse::<u64>().unwrap();

            (time, record_distance)
        })
        .collect()
}

fn get_amount_of_wins((time_ref, record_distance_ref): &(u64, u64)) -> u64 {
    let time = *time_ref;
    let record_distance = *record_distance_ref;

    let mut charge_duration = 1;
    let mut amount_of_wins = 0;

    while charge_duration < time {
        let moving_duration = time - charge_duration;
        let distance = moving_duration * charge_duration;

        if distance > record_distance {
            amount_of_wins += 1;
        } else if amount_of_wins != 0 {
            break;
        }

        charge_duration += 1
    }

    amount_of_wins
}

fn parse_race(input: &str) -> (u64, u64) {
    let space_regex = Regex::new(r"\s+").unwrap();
    let mut lines = input.lines();
    let time_string = lines.next().unwrap().split(':').nth(1).unwrap().trim();
    let distance_string = lines.next().unwrap().split(':').nth(1).unwrap().trim();

    let time = space_regex
        .split(time_string)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = space_regex
        .split(distance_string)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    (time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
