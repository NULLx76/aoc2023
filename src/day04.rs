use itertools::Itertools;

#[derive(Debug, Clone, Hash)]
struct Card {
    id: usize,
    winning: Vec<u64>,
    numbers: Vec<u64>,
}

impl Card {
    fn count_wins(&self) -> usize {
        self.numbers.iter().filter(|n| self.winning.contains(n)).count()
    }
}

fn parse(input: String) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(':').unwrap();
            let id = l.strip_prefix("Card").unwrap().trim().parse().unwrap();
            let (winning, numbers) = r
                .split('|')
                .map(|el| {
                    el.trim()
                        .split(' ')
                        .filter_map(|n| n.parse().ok())
                        .collect()
                })
                .collect_tuple()
                .unwrap();

            Card {
                id,
                numbers,
                winning,
            }
        })
        .collect_vec()
}

pub fn part1(input: String) -> usize {
    let lines = parse(input);

    lines
        .iter()
        .map(Card::count_wins)
        .map(|n| {
            if n == 0 {
                0
            } else {
                usize::pow(2, n as u32 - 1)
            }
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    let input = parse(input);

    let map = input
        .iter()
        .map(Card::count_wins)
        .collect_vec();

    let mut c = 0;

    let mut cards: Vec<usize> = input.iter().map(|c| c.id).collect();

    while let Some(card) = cards.pop() {
        let n = map[card - 1];

        for i in 1..=n {
            cards.push(card + i);
        }

        c += 1;
    }

    c
}

#[cfg(test)]
mod tests {
    use crate::{
        day04::parse,
        day04::{part1, part2},
        input, input_example,
    };

    #[test]
    fn test_parse() {
        parse(input!(4));
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(part1(input_example!(4, 1)), 13);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(input!(4)), 27454);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(part2(input_example!(4, 1)), 30);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(input!(4)), 6857330);
    }
}
