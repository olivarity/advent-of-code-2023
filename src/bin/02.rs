advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .enumerate()
            .filter(|(_, line)| check_valid_game(&line))
            .fold(0, |a, (i, _)| a + i as u32 + 1),
    )
}

fn check_valid_game(line: &str) -> bool {
    let game = line.split_once(':').unwrap_or_default().1;
    game.split(';').all(check_valid_round)
}

fn check_valid_round(round: &str) -> bool {
    round.split(',').all(check_valid_count)
}

fn check_valid_count(count: &str) -> bool {
    let mut itr = count.trim().split_whitespace();
    let quantity: u32 = itr.next().unwrap().parse().unwrap();

    quantity
        <= match itr.next().unwrap() {
            "red" => 12,
            "green" => 13,
            "blue" => 14,
            _ => 0,
        }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
