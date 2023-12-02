pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .chars()
                .filter_map(|f| f.to_string().parse::<u64>().ok());

            let first = nums.next().unwrap();
            let last = nums.last().unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

fn numbers() -> Vec<(String, u8)> {
    let mut numbers: Vec<_> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .map(|(a, b)| ((*a).to_string(), *b))
    .collect();
    numbers.extend((1..=9).map(|n| (n.to_string(), n)));

    numbers
}

pub fn part2(input: &str) -> u64 {
    let lookup = numbers();
    input
        .lines()
        .map(|line| {
            let pair = reduce(&lookup, Pair::Empty, line);
            pair.as_64()
        })
        .sum()
}

#[derive(Clone, Copy)]
enum Pair {
    Empty,
    One(u8),
    Two(u8, u8),
}

impl Pair {
    fn push(self, n: u8) -> Self {
        match self {
            Pair::Empty => Pair::One(n),
            Pair::One(o) | Pair::Two(o, _) => Pair::Two(o, n),
        }
    }

    fn as_64(self) -> u64 {
        match self {
            Pair::Empty => unreachable!(),
            Pair::One(o) => o as u64 * 10 + o as u64,
            Pair::Two(l, r) => l as u64 * 10 + r as u64,
        }
    }
}

fn reduce(lookup: &[(String, u8)], p: Pair, input: &str) -> Pair {
    for (prefix, n) in lookup {
        if let Some(next) = input.strip_prefix(prefix) {
            let p = p.push(*n);
            if prefix.len() > 1 {
                return reduce(lookup, p, input.split_at(prefix.len() - 1).1);
            } 
            
            return reduce(lookup, p, next);
        }
    }

    if input.is_empty() {
        p
    } else {
        reduce(lookup, p, input.split_at(1).1)
    }
}

#[cfg(test)]
mod test {
    use crate::day01::{part1, part2};

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("../inputs/day01_example")), 142);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(include_str!("../inputs/day01")), 55172);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(include_str!("../inputs/day01")), 54925);
    }

    #[test]
    fn example_2() {
        assert_eq!(part2(include_str!("../inputs/day01_example2")), 281);
    }
}
