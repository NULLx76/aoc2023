use itertools::Itertools;

fn parse(input: String) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

pub fn part1(input: String) -> usize {
    let lines = parse(input);

    0
}

pub fn part2(input: String) -> u64 {
    let input = parse(input);

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{input, input_example};
    const DAY: usize = 5;

    #[test]
    fn test_parse() {
        parse(input!(DAY));
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(part1(input_example!(DAY, 1)), 0);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(input!(DAY)), 0);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(part2(input_example!(DAY, 1)), 0);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(input!(DAY)), 0);
    }
}
