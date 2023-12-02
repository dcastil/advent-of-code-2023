advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let game = parse_game(line);

        if is_game_possible(&game) {
            sum += game.id;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let game = parse_game(line);
        let minimum_set = get_minimum_set(&game.sets);
        let minimum_set_power = minimum_set.red * minimum_set.green * minimum_set.blue;

        sum += minimum_set_power;
    }

    Some(sum)
}

fn parse_game(line: &str) -> Game {
    let mut line_iterator = line.split(": ");
    let game_string = line_iterator.next().unwrap();
    let sets_string = line_iterator.next().unwrap();

    let id = game_string
        .split(' ')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let sets = sets_string.split("; ").map(parse_set).collect();

    Game { id, sets }
}

fn parse_set(set_string: &str) -> GameSet {
    let mut set = GameSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    for color_string in set_string.split(", ") {
        let mut color_iterator = color_string.split(' ');
        let color_value = color_iterator.next().unwrap().parse::<u32>().unwrap();

        match color_iterator.next().unwrap() {
            "red" => set.red = color_value,
            "green" => set.green = color_value,
            "blue" => set.blue = color_value,
            _ => panic!("Unknown color"),
        }
    }

    set
}

fn is_game_possible(game: &Game) -> bool {
    game.sets.iter().all(is_set_possible)
}

fn is_set_possible(set: &GameSet) -> bool {
    set.red <= 12 && set.green <= 13 && set.blue <= 14
}

fn get_minimum_set(sets: &Vec<GameSet>) -> GameSet {
    let mut minimum_set = GameSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in sets {
        if set.red > minimum_set.red {
            minimum_set.red = set.red;
        }

        if set.green > minimum_set.green {
            minimum_set.green = set.green;
        }

        if set.blue > minimum_set.blue {
            minimum_set.blue = set.blue;
        }
    }

    minimum_set
}

struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
