use regex::Regex;
use util::{Part, get_part, read_input};

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

const PATTERN: &str = r"([a-z ]+) (\d+),(\d+) through (\d+),(\d+)";

fn main() {
    let input = read_input("days/day06/data/input.txt");
    let brightness = get_part() == Part::Two;

    let mut matrix = vec![vec![0u8; 1000]; 1000];
    decorate(input, &mut matrix, brightness);

    if brightness {
        println!(
            "Part 2: Lights lit up with #{} brightness",
            calc_brightness(&matrix)
        );
    } else {
        println!("Part 1: Lights lit up #{}", calc_lit_up(&matrix));
    }
}

fn calc_lit_up(matrix: &Vec<Vec<u8>>) -> usize {
    matrix.iter().flatten().filter(|i| **i > 0).count()
}

fn calc_brightness(matrix: &Vec<Vec<u8>>) -> u32 {
    matrix.iter().flatten().map(|i| *i as u32).sum()
}

fn decorate<T: AsRef<str>>(input: T, matrix: &mut Vec<Vec<u8>>, brightness: bool) {
    let re = Regex::new(PATTERN).unwrap();

    input
        .as_ref()
        .lines()
        .map(|ln| into_action(&re, ln))
        .for_each(|act| {
            for x in act.1.x..=act.2.x {
                for y in act.1.y..=act.2.y {
                    match act.0 {
                        Action::Toggle => {
                            if brightness {
                                matrix[x][y] += 2
                            } else {
                                matrix[x][y] = !(matrix[x][y] != 0) as u8
                            }
                        }
                        Action::TurnOn => {
                            if brightness {
                                matrix[x][y] += 1
                            } else {
                                matrix[x][y] = 1
                            }
                        }
                        Action::TurnOff => {
                            if brightness {
                                if matrix[x][y] > 0 {
                                    matrix[x][y] -= 1
                                }
                            } else {
                                matrix[x][y] = 0
                            }
                        }
                    }
                }
            }
        });
}

fn into_action<T: AsRef<str>>(re: &Regex, data: T) -> (Action, Point, Point) {
    if let Some(x) = re.captures(data.as_ref()) {
        let p1 = Point {
            x: x[2].parse::<usize>().unwrap(),
            y: x[3].parse::<usize>().unwrap(),
        };
        let p2 = Point {
            x: x[4].parse::<usize>().unwrap(),
            y: x[5].parse::<usize>().unwrap(),
        };
        match &x[1] {
            "toggle" => (Action::Toggle, p1, p2),
            "turn on" => (Action::TurnOn, p1, p2),
            "turn off" => (Action::TurnOff, p1, p2),
            _ => panic!("Help!"),
        }
    } else {
        panic!("Help!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {
        let re = Regex::new(PATTERN).unwrap();
        assert_eq!(
            into_action(&re, "turn on 1461,550 through 64,900"),
            (
                Action::TurnOn,
                Point { x: 1461, y: 550 },
                Point { x: 64, y: 900 }
            )
        );
        assert_eq!(
            into_action(&re, "turn off 370,39 through 425,839"),
            (
                Action::TurnOff,
                Point { x: 370, y: 39 },
                Point { x: 425, y: 839 }
            )
        );
        assert_eq!(
            into_action(&re, "toggle 461,550 through 564,900"),
            (
                Action::Toggle,
                Point { x: 461, y: 550 },
                Point { x: 564, y: 900 }
            )
        );
    }

    #[test]
    fn test_brightness() {
        let mut matrix = vec![vec![0u8; 100]; 100];
        decorate("turn on 0,0 through 0,0", &mut matrix, true);
        assert_eq!(calc_brightness(&matrix), 1);

        let mut matrix = vec![vec![0u8; 100]; 100];
        decorate("toggle 0,0 through 99,99", &mut matrix, true);
        assert_eq!(calc_brightness(&matrix), 20000);
    }
}
