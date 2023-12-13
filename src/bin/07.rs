advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_bids_sum(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_bids_sum(input, true))
}

fn get_bids_sum(input: &str, has_jokers: bool) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(6);
            (
                get_hand_rank(&left[..5], has_jokers),
                right.parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    hands.sort_by_key(|(rank, _)| *rank);

    hands
        .iter()
        .enumerate()
        .fold(0, |sum, (index, (_, current_bid))| {
            sum + (index as u32 + 1) * current_bid
        })
}

fn get_hand_rank(hand: &str, has_jokers: bool) -> u32 {
    let kind_distribution = get_kind_distribution(hand, has_jokers);

    let mut hand_rank = get_hand_type_rank(kind_distribution);

    for card in hand.chars() {
        hand_rank <<= 4;
        hand_rank |= get_rank_for_card(&card, has_jokers);
    }

    hand_rank
}

fn get_kind_distribution(hand: &str, has_jokers: bool) -> [u32; 5] {
    let mut kind_values: [Option<(char, u32)>; 5] = [None; 5];
    let mut jokers_count = 0;

    for card in hand.chars() {
        if card == 'J' && has_jokers {
            jokers_count += 1;
            continue;
        }

        for option in kind_values.iter_mut() {
            if let Some((current_card, count)) = option {
                if *current_card == card {
                    *count += 1;
                    break;
                }
            } else {
                *option = Some((card, 1));
                break;
            }
        }
    }

    let mut kind_distribution = kind_values.map(|option| option.map_or(0, |(_, count)| count));

    kind_distribution.sort_by(|a, b| b.cmp(a));

    if jokers_count > 0 {
        kind_distribution[0] += jokers_count;
    }

    kind_distribution
}

fn get_hand_type_rank(kind_distribution: [u32; 5]) -> u32 {
    match kind_distribution {
        [5, 0, 0, 0, 0] => 0x6,
        [4, 1, 0, 0, 0] => 0x5,
        [3, 2, 0, 0, 0] => 0x4,
        [3, 1, 1, 0, 0] => 0x3,
        [2, 2, 1, 0, 0] => 0x2,
        [2, 1, 1, 1, 0] => 0x1,
        [1, 1, 1, 1, 1] => 0x0,
        _ => panic!("Invalid kind distribution"),
    }
}

fn get_rank_for_card(card: &char, has_jokers: bool) -> u32 {
    match card {
        'A' => 0xd,
        'K' => 0xc,
        'Q' => 0xb,
        'J' => {
            if has_jokers {
                0x0
            } else {
                0xa
            }
        }
        'T' => 0x9,
        '9' => 0x8,
        '8' => 0x7,
        '7' => 0x6,
        '6' => 0x5,
        '5' => 0x4,
        '4' => 0x3,
        '3' => 0x2,
        '2' => 0x1,
        _ => panic!("Invalid card"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
