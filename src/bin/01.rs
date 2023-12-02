advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                let mut digits = s.chars().filter_map(|c| c.to_digit(10));
                let first = digits.next().unwrap_or_default();
                let last = digits.last();

                match last {
                    Some(n) => first * 10 + n,
                    None => first * 11,
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_line).sum())
}

fn parse_line(line: &str) -> u32 {
    let first = line
        .char_indices()
        .find_map(|(i, c)| c.to_digit(10).or(starts_with_digit_word(&line[i..])));
    let last = line
        .char_indices()
        .rev()
        .find_map(|(i, c)| c.to_digit(10).or(starts_with_digit_word(&line[i..])));

    first.unwrap_or_default() * 10 + last.unwrap_or_default()
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn starts_with_digit_word(s: &str) -> Option<u32> {
    DIGITS
        .iter()
        .enumerate()
        .find_map(|(index, digit)| s.starts_with(digit).then_some(index as u32 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
