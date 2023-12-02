#[derive(Default, Copy, Clone, Debug)]
struct Grab {
    red: u64,
    blue: u64,
    green: u64,
}

impl Grab {
    fn power(self) -> u64 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    grabs: Vec<Grab>,
}

impl Game {
    fn highest_grab(&self) -> Grab {
        self.grabs.iter().fold(Grab::default(), |acc, el| Grab {
            red: if acc.red > el.red { acc.red } else { el.red },
            green: if acc.green > el.green {
                acc.green
            } else {
                el.green
            },
            blue: if acc.blue > el.blue {
                acc.blue
            } else {
                el.blue
            },
        })
    }
}

fn parse(input: String) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(':').unwrap();
            let id = l.strip_prefix("Game ").unwrap().parse().unwrap();

            let grabs = r
                .split(';')
                .map(|grab| {
                    grab.split(',').fold(Grab::default(), |mut acc, colour| {
                        let (n, c) = colour.trim().split_once(' ').unwrap();
                        let n: u64 = n.parse().unwrap();
                        match c {
                            "red" => acc.red += n,
                            "blue" => acc.blue += n,
                            "green" => acc.green += n,
                            _ => unreachable!(),
                        }

                        acc
                    })
                })
                .collect();

            Game { id, grabs }
        })
        .collect()
}

pub fn part1(input: String) -> u64 {
    let (r, g, b) = (12, 13, 14);

    parse(input)
        .into_iter()
        .filter_map(|game| {
            let grab = game.highest_grab();
            (grab.red <= r && grab.green <= g && grab.blue <= b).then_some(game.id)
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    parse(input)
        .iter()
        .map(Game::highest_grab)
        .map(Grab::power)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        day02::parse,
        day02::{part1, part2},
        input, input_example,
    };

    #[test]
    fn test_parse() {
        parse(input_example!(2, 1));
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(part1(input_example!(2, 1)), 8);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part1(input!(2)), 2449);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(part2(input_example!(2, 1)), 2286);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(input!(2)), 63981);
    }
}
