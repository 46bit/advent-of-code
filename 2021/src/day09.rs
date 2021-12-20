use std::collections::{HashMap, HashSet};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

#[aoc(day9, part1)]
fn part1(input: &str) -> i64 {
    let mut yx_grid = [[0; WIDTH]; HEIGHT];
    for (y, line) in input.lines().enumerate() {
        for (x, char_) in line.chars().enumerate() {
            let n = format!("{}", char_).parse::<u8>().unwrap();
            yx_grid[y][x] = n;
        }
    }

    let mut low_points = HashSet::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y > 0 && yx_grid[y - 1][x] <= yx_grid[y][x] {
                continue;
            }
            if y + 1 < HEIGHT && yx_grid[y + 1][x] <= yx_grid[y][x] {
                continue;
            }
            if x > 0 && yx_grid[y][x - 1] <= yx_grid[y][x] {
                continue;
            }
            if x + 1 < WIDTH && yx_grid[y][x + 1] <= yx_grid[y][x] {
                continue;
            }
            low_points.insert((y, x));
        }
    }

    let mut summed_risk_levels = 0;
    for low_point in low_points {
        summed_risk_levels += yx_grid[low_point.0][low_point.1] as i64 + 1;
    }
    return summed_risk_levels;
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i64 {
    let mut yx_grid = [[0; WIDTH]; HEIGHT];
    for (y, line) in input.lines().enumerate() {
        for (x, char_) in line.chars().enumerate() {
            let n = format!("{}", char_).parse::<u8>().unwrap();
            yx_grid[y][x] = n;
        }
    }

    let mut low_points = HashSet::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y > 0 && yx_grid[y - 1][x] <= yx_grid[y][x] {
                continue;
            }
            if y + 1 < HEIGHT && yx_grid[y + 1][x] <= yx_grid[y][x] {
                continue;
            }
            if x > 0 && yx_grid[y][x - 1] <= yx_grid[y][x] {
                continue;
            }
            if x + 1 < WIDTH && yx_grid[y][x + 1] <= yx_grid[y][x] {
                continue;
            }
            low_points.insert((y, x));
        }
    }

    let mut regions = HashMap::new();
    for low_point in low_points {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = Vec::new();
        visited.insert(low_point);
        queue.push(low_point);
        loop {
            let current = match queue.pop() {
                Some(v) => v,
                None => break,
            };
            let y = current.0;
            let x = current.1;

            let mut neighbours = vec![];
            if y > 0 {
                neighbours.push((y - 1, x));
            }
            if y + 1 < HEIGHT {
                neighbours.push((y + 1, x));
            }
            if x > 0 {
                neighbours.push((y, x - 1));
            }
            if x + 1 < WIDTH {
                neighbours.push((y, x + 1));
            }
            queue.append(
                &mut neighbours
                    .into_iter()
                    .filter(|n| {
                        if visited.contains(n) {
                            return false;
                        }
                        let valid = yx_grid[n.0][n.1] < 9;
                        if valid {
                            visited.insert(*n);
                        }
                        valid
                    })
                    .collect(),
            );
        }
        regions.insert(low_point, visited);
    }
    println!("{:?}", regions);

    let mut region_sizes: Vec<(_, _)> = regions.iter().map(|(k, v)| (k, v.len() as i64)).collect();
    region_sizes.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));
    println!("{:?}", region_sizes);

    return region_sizes[region_sizes.len() - 1].1
        * region_sizes[region_sizes.len() - 2].1
        * region_sizes[region_sizes.len() - 3].1;
}
