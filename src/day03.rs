fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

fn numbers_in_row(row: &[char]) -> Vec<(usize, usize, u64)> {
    let mut numbers = Vec::new();

    let mut stack = Vec::new();
    let mut start = None;
    for (y, ch) in row.iter().enumerate() {
        if ch.is_numeric() {
            if start.is_none() {
                start = Some(y);
            }
            stack.push(*ch);
        } else if !stack.is_empty() {
            let num = stack.into_iter().collect::<String>();
            numbers.push((start.unwrap(), y - 1, num.parse::<u64>().unwrap()));
            start = None;
            stack = Vec::new();
        }
    }

    if !stack.is_empty() {
        let num = stack.into_iter().collect::<String>();
        numbers.push((start.unwrap(), row.len() - 1, num.parse::<u64>().unwrap()));
    }

    numbers
}

pub fn part1(input: String) -> u64 {
    let lines = parse(input);

    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let lines = &lines;
            numbers_in_row(line)
                .into_iter()
                .filter_map(move |(l, r, n)| symbol_adjacent(lines, y, l, r).then_some(n))
        })
        .sum()
}

#[inline]
pub fn is_symbol(ch: &char) -> bool {
    ch.is_ascii_punctuation() && ch != &'.'
}

fn symbol_adjacent(map: &[Vec<char>], row: usize, start: usize, end: usize) -> bool {
    let check_row = |row: &Vec<char>| {
        (start..=end).map(|i| &row[i]).any(is_symbol)
            || start > 0 && is_symbol(&row[start - 1])
            || row.get(end + 1).is_some_and(is_symbol)
    };

    return start > 0 && map[row].get(start - 1).is_some_and(is_symbol)
        || map[row].get(end + 1).is_some_and(is_symbol)
        || row > 0 && map.get(row - 1).is_some_and(check_row)
        || map.get(row + 1).is_some_and(check_row);
}

fn adjacent_numbers(map: &[Vec<char>], x: usize, y: usize) -> Vec<u64> {
    let filter_map = |(l, r, n)| {
        ((l..=r).contains(&x.wrapping_sub(1)) || (l..=r).contains(&x) || (l..=r).contains(&(x + 1)))
            .then_some(n)
    };

    let mut nums: Vec<_> = numbers_in_row(&map[y])
        .into_iter()
        .filter_map(filter_map)
        .collect();

    if y > 0 {
        nums.extend(
            numbers_in_row(&map[y - 1])
                .into_iter()
                .filter_map(filter_map),
        );
    }

    if y + 1 < map[y].len() {
        nums.extend(
            numbers_in_row(&map[y + 1])
                .into_iter()
                .filter_map(filter_map),
        );
    }

    nums
}

pub fn part2(input: String) -> u64 {
    let input = parse(input);
    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let input = &input;
            line.iter().enumerate().filter_map(move |(x, &ch)| {
                (ch == '*')
                    .then(|| {
                        let nums = adjacent_numbers(input, x, y);
                        (nums.len() == 2).then(|| nums[0] * nums[1])
                    })
                    .flatten()
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        day03::parse,
        day03::{part1, part2, symbol_adjacent},
        input, input_example, input_ext,
    };

    #[test]
    fn test_adj() {
        let input = parse([".....", ".123.", "....%"].join("\n"));
        assert!(symbol_adjacent(&input, 1, 1, 3))
    }

    #[test]
    fn test_parse() {
        parse(input_example!(3, 1));
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(part1(input_example!(3, 1)), 4361);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(input!(3)), 527_364);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(part2(input_example!(3, 1)), 467_835);
    }

    #[test]
    fn test_example_p2_laura() {
        assert_eq!(part1(input_ext!(3, "_laura")), 556_367);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(input!(3)), 79026871);
    }
}
