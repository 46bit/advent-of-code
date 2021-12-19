use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Move {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Forward(distance) => write!(f, "forward {}", distance),
            Move::Up(distance) => write!(f, "up {}", distance),
            Move::Down(distance) => write!(f, "down {}", distance),
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(" ");
        let name = some_or_return!(tokens.next(), "line was empty".to_string());
        let distance = some_or_return!(tokens.next(), "line had too few tokens".to_string());
        let distance_number = ok_or_return_s!(distance.parse::<i32>());
        if let Some(_) = tokens.next() {
            return Err("line had too many tokens".to_string());
        }

        match name {
            "forward" => Ok(Move::Forward(distance_number)),
            "up" => Ok(Move::Up(distance_number)),
            "down" => Ok(Move::Down(distance_number)),
            _ => Err("unexpected move name".to_string()),
        }
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    let moves = input.lines().map(Move::from_str).map(Result::unwrap);
    // let recreated_input = moves
    //     .map(|m| format!("{}", m))
    //     .collect::<Vec<_>>()
    //     .join("\n");
    // assert_eq!(input, recreated_input);

    let mut x = 0;
    let mut y = 0;
    for m in moves {
        match m {
            Move::Forward(distance) => {
                x += distance;
            }
            Move::Up(distance) => {
                y -= distance;
            }
            Move::Down(distance) => {
                y += distance;
            }
        }
    }
    x * y
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    let moves = input.lines().map(Move::from_str).map(Result::unwrap);

    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for m in moves {
        match m {
            Move::Forward(distance) => {
                x += distance;
                y += aim * distance;
            }
            Move::Up(distance) => {
                aim -= distance;
            }
            Move::Down(distance) => {
                aim += distance;
            }
        }
    }
    x * y
}
