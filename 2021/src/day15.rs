use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    let mut movement_costs = [[0; WIDTH]; HEIGHT];
    for (y, line) in input.lines().enumerate() {
        for (x, char_) in line.chars().enumerate() {
            movement_costs[y][x] = format!("{}", char_).parse().unwrap();
        }
    }
    let path = a_star(
        (0, 0),
        (HEIGHT - 1, WIDTH - 1),
        SimpleMovementCosts {
            yx_grid: movement_costs,
        },
    )
    .unwrap();
    let cost = path.iter().map(|v| movement_costs[v.0][v.1]).sum::<usize>()
        - movement_costs[HEIGHT - 1][WIDTH - 1];
    println!("len={} cost={} route={:?}", path.len(), cost, path);
    return cost;
}

fn a_star(
    start: (usize, usize),
    end: (usize, usize),
    movement_costs: impl MovementCosts,
) -> Option<Vec<(usize, usize)>> {
    let mut open = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut came_from = HashMap::new();

    let h = |(y, x): (usize, usize)| end.1.abs_diff(x) + end.0.abs_diff(y);
    open.push(OpenItem((0, 0), h(start)));
    g_score.insert((0, 0), 0);

    while let Some(OpenItem(current, _)) = open.pop() {
        //println!("{:?}", current);
        if current == end {
            return Some(reconstruct_path(came_from, current));
        }

        let mut neighbours = vec![];
        if current.0 > 0 {
            neighbours.push((current.0 - 1, current.1));
        }
        if current.1 > 0 {
            neighbours.push((current.0, current.1 - 1));
        }
        if current.0 + 1 < movement_costs.total_height() {
            neighbours.push((current.0 + 1, current.1));
        }
        if current.1 + 1 < movement_costs.total_width() {
            neighbours.push((current.0, current.1 + 1));
        }
        for neighbour in neighbours {
            let new_g_score = g_score[&current] + movement_costs.get_cost(neighbour);
            if !g_score.contains_key(&neighbour) || new_g_score < g_score[&neighbour] {
                open.push(OpenItem(neighbour, new_g_score + h(neighbour)));
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, new_g_score);
            }
        }
    }

    return None;
}

trait MovementCosts {
    fn get_cost(&self, point: (usize, usize)) -> usize;
    fn total_height(&self) -> usize;
    fn total_width(&self) -> usize;
}

#[derive(Copy, Clone, Debug)]
struct SimpleMovementCosts<const WIDTH: usize, const HEIGHT: usize> {
    yx_grid: [[usize; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> MovementCosts for SimpleMovementCosts<WIDTH, HEIGHT> {
    fn get_cost(&self, point: (usize, usize)) -> usize {
        self.yx_grid[point.0][point.1]
    }
    fn total_height(&self) -> usize {
        HEIGHT
    }

    fn total_width(&self) -> usize {
        WIDTH
    }
}

#[derive(Copy, Clone, Debug)]
struct Part2MovementCosts<const WIDTH: usize, const HEIGHT: usize> {
    yx_grid: [[usize; WIDTH]; HEIGHT],
    tiles: usize,
}

impl<const WIDTH: usize, const HEIGHT: usize> MovementCosts for Part2MovementCosts<WIDTH, HEIGHT> {
    fn get_cost(&self, point: (usize, usize)) -> usize {
        let mut base_cost = self.yx_grid[point.0 % HEIGHT][point.1 % WIDTH];
        for _ in 0..(point.0 / HEIGHT) {
            base_cost += 1;
            if base_cost > 9 {
                base_cost = 1;
            }
        }
        for _ in 0..(point.1 / WIDTH) {
            base_cost += 1;
            if base_cost > 9 {
                base_cost = 1;
            }
        }
        base_cost
    }

    fn total_height(&self) -> usize {
        HEIGHT * self.tiles
    }

    fn total_width(&self) -> usize {
        WIDTH * self.tiles
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct OpenItem((usize, usize), usize);

impl Ord for OpenItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for OpenItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn reconstruct_path(
    came_from: HashMap<(usize, usize), (usize, usize)>,
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path = vec![];
    let mut current = Some(&end);
    while let Some(v) = current {
        path.push(*v);
        current = came_from.get(v);
    }
    return path;
}

fn print_path(path: Vec<(usize, usize)>, movement_costs: impl MovementCosts) {
    let path: HashSet<_> = path.into_iter().collect();
    for y in 0..movement_costs.total_height() {
        for x in 0..movement_costs.total_width() {
            if path.contains(&(y, x)) {
                print!("{}", movement_costs.get_cost((y, x)));
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

const TILES: usize = 5;

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let mut movement_costs = [[0; WIDTH]; HEIGHT];
    for (y, line) in input.lines().enumerate() {
        for (x, char_) in line.chars().enumerate() {
            movement_costs[y][x] = format!("{}", char_).parse().unwrap();
        }
    }
    let movement_costs = Part2MovementCosts {
        yx_grid: movement_costs,
        tiles: TILES,
    };
    let path = a_star(
        (0, 0),
        (TILES * HEIGHT - 1, TILES * WIDTH - 1),
        movement_costs,
    )
    .unwrap();
    let cost = path
        .iter()
        .map(|v| movement_costs.get_cost(*v))
        .sum::<usize>()
        - movement_costs.get_cost((0, 0));
    println!("len={} cost={} route={:?}", path.len(), cost, path);
    print_path(path, movement_costs);
    return cost;
}
