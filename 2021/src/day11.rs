use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct OctopusGrid {
    yx_energy_grid: [[u8; 10]; 10],
}

impl OctopusGrid {
    fn new() -> OctopusGrid {
        OctopusGrid {
            yx_energy_grid: [[0; 10]; 10],
        }
    }

    fn tick(&mut self) -> u64 {
        for y in 0..10 {
            for x in 0..10 {
                self.yx_energy_grid[y][x] += 1;
            }
        }

        let mut flashes = 0;
        let mut flashed = HashSet::new();
        let mut still_flashing = true;
        while still_flashing {
            still_flashing = false;
            for y in 0..10 {
                for x in 0..10 {
                    if flashed.contains(&(y, x)) {
                        continue;
                    }
                    if self.yx_energy_grid[y][x] > 9 {
                        flashes += 1;
                        still_flashing = true;
                        flashed.insert((y, x));

                        let mut neighbours = vec![];
                        if y > 0 && x > 0 {
                            neighbours.push((y - 1, x - 1));
                        }
                        if y + 1 < 10 && x + 1 < 10 {
                            neighbours.push((y + 1, x + 1));
                        }
                        if y > 0 && x + 1 < 10 {
                            neighbours.push((y - 1, x + 1));
                        }
                        if x > 0 && y + 1 < 10 {
                            neighbours.push((y + 1, x - 1));
                        }
                        if y > 0 {
                            neighbours.push((y - 1, x));
                        }
                        if y + 1 < 10 {
                            neighbours.push((y + 1, x));
                        }
                        if x > 0 {
                            neighbours.push((y, x - 1));
                        }
                        if x + 1 < 10 {
                            neighbours.push((y, x + 1));
                        }
                        for neighbour in neighbours {
                            self.yx_energy_grid[neighbour.0][neighbour.1] += 1;
                        }
                    }
                }
            }
        }

        for (y, x) in flashed {
            self.yx_energy_grid[y][x] = 0;
        }

        return flashes;
    }
}

impl fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..10 {
            for x in 0..10 {
                ok_or_return!(write!(f, "{}", self.yx_energy_grid[y][x]));
            }
            if y < 9 {
                ok_or_return!(write!(f, "\n"));
            }
        }
        Ok(())
    }
}

impl FromStr for OctopusGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octopus_grid = OctopusGrid::new();
        for (y, line) in s.lines().enumerate() {
            for (x, char_) in line.chars().enumerate() {
                let n = ok_or_return_s!(format!("{}", char_).parse());
                octopus_grid.yx_energy_grid[y][x] = n;
            }
        }
        Ok(octopus_grid)
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> u64 {
    let mut octopus_grid = OctopusGrid::from_str(input).unwrap();
    //assert_eq!(format!("{}", octopus_grid), input);
    println!("{}", octopus_grid);

    let mut flashes = 0;
    for _step in 1..=100 {
        flashes += octopus_grid.tick();
        println!("{}: {}", _step, flashes);
        println!("{}", octopus_grid);
    }

    return flashes;
}

#[aoc(day11, part2)]
fn part2(input: &str) -> i32 {
    let mut octopus_grid = OctopusGrid::from_str(input).unwrap();
    //assert_eq!(format!("{}", octopus_grid), input);
    println!("{}", octopus_grid);

    for _step in 1.. {
        let flashes = octopus_grid.tick();
        println!("{}: {}", _step, flashes);
        //println!("{}", octopus_grid);
        if flashes == 100 {
            return _step;
        }
    }
    unreachable!();
}
