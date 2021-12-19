use std::cmp::max;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(",");
        let x = some_or_return!(tokens.next(), "not enough tokens".to_string());
        let y = some_or_return!(tokens.next(), "not enough tokens".to_string());
        if tokens.next().is_some() {
            return Err("too many tokens".to_string());
        }
        Ok(Coordinate {
            x: ok_or_return_s!(x.parse::<usize>()),
            y: ok_or_return_s!(y.parse::<usize>()),
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Coordinate,
    end: Coordinate,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn increment_on_yx_grid<const H: usize, const W: usize>(
        &self,
        yx_grid: &mut [[u16; W]; H],
    ) -> Result<(), String> {
        if self.start.y > W || self.end.y > W {
            return Err("outside y bounds".to_string());
        }
        if self.start.x > W || self.end.x > W {
            return Err("outside x bounds".to_string());
        }

        let dy = (self.end.y as isize) - (self.start.y as isize);
        let dx = (self.end.x as isize) - (self.start.x as isize);
        if dy != 0 && dx != 0 && dy.abs() != dx.abs() {
            return Err("only 45-degree diagonals supported".to_string());
        }
        let length = max(dy.abs(), dx.abs());
        if length == 0 {
            return Ok(());
        }
        let unit_dy = dy / length;
        let unit_dx = dx / length;
        let mut y = self.start.y;
        let mut x = self.start.x;
        for _ in 0..=length {
            yx_grid[y][x] += 1;
            y = (y as isize + unit_dy) as usize;
            x = (x as isize + unit_dx) as usize;
        }
        Ok(())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(" -> ");
        let start = some_or_return!(tokens.next(), "not enough tokens".to_string());
        let end = some_or_return!(tokens.next(), "not enough tokens".to_string());
        if tokens.next().is_some() {
            return Err("too many tokens".to_string());
        }
        Ok(Line {
            start: Coordinate::from_str(start)?,
            end: Coordinate::from_str(end)?,
        })
    }
}

const GRID_HEIGHT: usize = 1000;
const GRID_WIDTH: usize = 1000;

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    let lines = input.lines().map(Line::from_str).map(Result::unwrap);
    // let recreated_input = lines
    //     .clone()
    //     .map(|l| format!("{}", l))
    //     .collect::<Vec<_>>()
    //     .join("\n");
    // assert_eq!(recreated_input, input);

    let mut yx_grid = [[0; GRID_WIDTH]; GRID_HEIGHT];
    for (_i, line) in lines.enumerate() {
        if line.is_diagonal() {
            continue;
        }
        //println!("{}: {}", i, line);
        line.increment_on_yx_grid(&mut yx_grid).unwrap();
    }

    let mut points_greater_than_1 = 0;
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if yx_grid[y][x] > 1 {
                points_greater_than_1 += 1;
            }
        }
    }
    return points_greater_than_1;
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    let lines = input.lines().map(Line::from_str).map(Result::unwrap);

    let mut yx_grid = [[0; GRID_WIDTH]; GRID_HEIGHT];
    for (_i, line) in lines.enumerate() {
        //println!("{}: {}", i, line);
        line.increment_on_yx_grid(&mut yx_grid).unwrap();
    }

    let mut points_greater_than_1 = 0;
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if yx_grid[y][x] > 1 {
                points_greater_than_1 += 1;
            }
        }
    }
    return points_greater_than_1;
}
