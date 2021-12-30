use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scanner {
    id: String,
    beacons: Vec<Coord>,
    beacon_distances: HashMap<u64, (usize, usize)>,
}

impl Scanner {
    pub fn new(id: String, beacons: Vec<Coord>) -> Scanner {
        let mut scanner = Scanner { 
            id, 
            beacons, 
            beacon_distances: HashMap::new(),
        };
        scanner.recalculate_beacon_distances();
        scanner
    }

    pub fn recalculate_beacon_distances(&mut self) {
        self.beacon_distances = HashMap::new();
        for i in 0..self.beacons.len() {
            for j in 0..self.beacons.len() {
                if i == j {
                    continue;
                }
                let a = &self.beacons[i];
                let b = &self.beacons[j];
                self.beacon_distances.insert(a.distance_squared(b), (i, j));
            }
        }
    }

    pub fn match_beacons(&self, other: &Scanner) -> Vec<((usize, usize), (usize, usize))> {
        let mut matches = vec![];
        for (dist1, (i, j)) in &self.beacon_distances {
            for (dist2, (k, l)) in &other.beacon_distances {
                if dist1 == dist2 {
                    matches.push(((*i, *j), (*k, *l)));
                }
            }
        }
        return matches;
    }

    pub fn find_rotation(&self, other: &Scanner, matches: &Vec<((usize, usize), (usize, usize))>) -> Option<(Matrix, Coord)> {
        for matrix in Matrix::all_with_all_signs() {
            let mut offsets: HashMap<Coord, usize> = HashMap::new();
            for ((i, j), (k, l)) in matches {
                for a in [i, j] {
                    for b in [k, l] {
                        let ta = self.beacons[*a].apply_matrix(&matrix);
                        let offset = Coord {
                            x: other.beacons[*b].x - ta.x,
                            y: other.beacons[*b].y - ta.y,
                            z: other.beacons[*b].z - ta.z,
                        };
                        *offsets.entry(offset).or_default() += 1;
                    }
                }
            }

            if let Some((best_offset, occurences)) = offsets.into_iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)) {
                if occurences == matches.len() * 2 {
                    return Some((matrix, best_offset));
                }
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Transform {
    matrix: Matrix,
    signs: Coord,
    offset: Coord,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Matrix([[i64; 3]; 3]);

impl Matrix {
    fn all_with_all_signs() -> impl Iterator<Item = Matrix> {
        MATRICES.iter().map(|m| m.sign_variations()).flatten()
    }

    fn sign_variations(&self) -> Vec<Matrix> {
        let mut matrices = vec![];
        for sx in [1, -1] {
            for sy in [1, -1] {
                for sz in [1, -1] {
                    matrices.push(Matrix([
                        [sx * self.0[0][0], sx * self.0[0][1], sx * self.0[0][2]],
                        [sy * self.0[1][0], sy * self.0[1][1], sy * self.0[1][2]],
                        [sz * self.0[2][0], sz * self.0[2][1], sz * self.0[2][2]],
                    ]));
                }
            }
        }
        return matrices;
    }
}

pub const MATRICES: &[Matrix; 6] = &[
    // (x,y,z) = (x,y,z)
    Matrix([[1, 0, 0],
            [0, 1, 0],
            [0, 0, 1]]),
    // (x,y,z) = (y,x,z)
    Matrix([[0, 1, 0],
            [1, 0, 0],
            [0, 0, 1]]),
    // (x,y,z) = (x,z,y)
    Matrix([[1, 0, 0],
            [0, 0, 1],
            [0, 1, 0]]),
    // (x,y,z) = (z,y,x)
    Matrix([[0, 0, 1],
            [0, 1, 0],
            [1, 0, 0]]),
    // (x,y,z) = (y,z,x)
    Matrix([[0, 1, 0],
            [0, 0, 1],
            [1, 0, 0]]),
    // (x,y,z) = (z,x,y)
    Matrix([[0, 0, 1],
            [1, 0, 0],
            [0, 1, 0]]),
];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn apply_matrix(&self, matrix: &Matrix) -> Coord {
        Coord {
            x: matrix.0[0][0] * self.x + matrix.0[0][1] * self.y + matrix.0[0][2] * self.z,
            y: matrix.0[1][0] * self.x + matrix.0[1][1] * self.y + matrix.0[1][2] * self.z,
            z: matrix.0[2][0] * self.x + matrix.0[2][1] * self.y + matrix.0[2][2] * self.z,
        }
    }

    fn distance_squared(&self, other: &Coord) -> u64 {
        ((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)) as u64
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let mut scanners = vec![];
    let mut lines = input.lines();
    loop {
        let line = match lines.next() {
            Some(s) => s,
            None => break,
        };
        let space_separated: Vec<_> = line.split(" ").collect();
        assert_eq!(space_separated[0], "---");
        assert_eq!(space_separated[1], "scanner");
        assert_eq!(space_separated[3], "---");
        let id = space_separated[2].to_string();
        let mut beacons = vec![];
        loop {
            let line = match lines.next() {
                Some(s) => s,
                None => break,
            };
            if line.len() == 0 {
                break;
            }
            let comma_separated: Vec<_> = line.split(",").collect();
            let x: i64 = comma_separated[0].parse().unwrap();
            let y: i64 = comma_separated[1].parse().unwrap();
            let z: i64 = comma_separated[2].parse().unwrap();
            beacons.push(Coord {x, y, z});
        }
        scanners.push(Scanner::new(id, beacons));
    }
    //println!("{:?}", scanners);

    let mut transforms: HashMap<(usize, usize), (Matrix, Coord)> = HashMap::new();
    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            if i == j {
                continue;
            }
            let matches = scanners[i].match_beacons(&scanners[j]);
            if matches.len() < 12 {
                // if matches.len() > 0 {
                //     println!("[{} x {}] discarding match of size {} because <12", scanners[i].id, scanners[j].id, matches.len());
                // }
                continue;
            }
            if let Some((matrix, offset)) = scanners[i].find_rotation(&scanners[j], &matches) {
                //println!("[{} x {}] best: rotation={:?} offset={:?}", scanners[i].id, scanners[j].id, rotation, offset);
                transforms.insert((i, j), (matrix, offset));
            }
        }
    }
    println!("transforms = {:?}", transforms.keys().collect::<Vec<_>>());

    let mut reached = HashSet::new();
    reached.insert(0);
    let mut open = vec![0];
    let mut closed = HashSet::new();
    let mut came_from = HashMap::new();
    while let Some(current) = open.pop() {
        if closed.contains(&current) {
            continue;
        }
        closed.insert(current);
        reached.insert(current);

        for i in 0..scanners.len() {
            if i == current || reached.contains(&i) {
                continue;
            }
            if let Some(_) = transforms.get(&(current, i)) {
                reached.insert(i);
                if !closed.contains(&i) {
                    open.push(i);
                    came_from.insert(i, current);
                }
            }
        }
    }

    println!("{:?} {:?}", reached.len(), scanners.len());
    if reached.len() != scanners.len() {
        panic!("not all reached");
    }

    let mut beacons_in_scanner_0_coords = HashSet::new();
    for beacon in &scanners[0].beacons {
        beacons_in_scanner_0_coords.insert(*beacon);
    }
    for i in 1..scanners.len() {
        print!("{}: ", i);
        let mut beacons = scanners[i].beacons.clone();
        let mut current = i;
        loop {
            if let Some(prev) = came_from.get(&current) {
                print!("{}, ", prev);
                beacons = beacons.into_iter().map(|b| {
                    let (matrix, offset) = transforms[&(current, *prev)];
                    let v = b.apply_matrix(&matrix);
                    Coord {
                        x: v.x + offset.x,
                        y: v.y + offset.y,
                        z: v.z + offset.z,
                    }
                }).collect();
                current = *prev;
                if current == 0 {
                    break;
                }
            } else {
                break;
            }
        }
        assert_eq!(current, 0);
        for beacon in beacons {
            beacons_in_scanner_0_coords.insert(beacon);
        }
        println!();
    }
    //println!("beacons_in_scanner_0_coords = {:?}", beacons_in_scanner_0_coords);
    println!("beacons_in_scanner_0_coords.len() = {}", beacons_in_scanner_0_coords.len());

    for i in 0..scanners.len() {
        println!("{}:", scanners[i].id);
        for j in 0..scanners.len() {
            if i == j {
                continue;
            }
            if transforms.contains_key(&(i, j)) {
                println!("  - {}", scanners[j].id);
            }
        }
    }

    for b in &beacons_in_scanner_0_coords {
        println!("{},{},{}", b.x, b.y, b.z);
    }
    println!("beacons_in_scanner_0_coords.len() = {}", beacons_in_scanner_0_coords.len());

    return beacons_in_scanner_0_coords.len();
}

#[aoc(day19, part2)]
fn part2(input: &str) -> u64 {
    let mut scanners = vec![];
    let mut lines = input.lines();
    loop {
        let line = match lines.next() {
            Some(s) => s,
            None => break,
        };
        let space_separated: Vec<_> = line.split(" ").collect();
        assert_eq!(space_separated[0], "---");
        assert_eq!(space_separated[1], "scanner");
        assert_eq!(space_separated[3], "---");
        let id = space_separated[2].to_string();
        let mut beacons = vec![];
        loop {
            let line = match lines.next() {
                Some(s) => s,
                None => break,
            };
            if line.len() == 0 {
                break;
            }
            let comma_separated: Vec<_> = line.split(",").collect();
            let x: i64 = comma_separated[0].parse().unwrap();
            let y: i64 = comma_separated[1].parse().unwrap();
            let z: i64 = comma_separated[2].parse().unwrap();
            beacons.push(Coord {x, y, z});
        }
        scanners.push(Scanner::new(id, beacons));
    }
    //println!("{:?}", scanners);

    let mut transforms: HashMap<(usize, usize), (Matrix, Coord)> = HashMap::new();
    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            if i == j {
                continue;
            }
            let matches = scanners[i].match_beacons(&scanners[j]);
            if matches.len() < 12 {
                // if matches.len() > 0 {
                //     println!("[{} x {}] discarding match of size {} because <12", scanners[i].id, scanners[j].id, matches.len());
                // }
                continue;
            }
            if let Some((matrix, offset)) = scanners[i].find_rotation(&scanners[j], &matches) {
                //println!("[{} x {}] best: rotation={:?} offset={:?}", scanners[i].id, scanners[j].id, rotation, offset);
                transforms.insert((i, j), (matrix, offset));
            }
        }
    }
    println!("transforms = {:?}", transforms.keys().collect::<Vec<_>>());

    let mut reached = HashSet::new();
    reached.insert(0);
    let mut open = vec![0];
    let mut closed = HashSet::new();
    let mut came_from = HashMap::new();
    while let Some(current) = open.pop() {
        if closed.contains(&current) {
            continue;
        }
        closed.insert(current);
        reached.insert(current);

        for i in 0..scanners.len() {
            if i == current || reached.contains(&i) {
                continue;
            }
            if let Some(_) = transforms.get(&(current, i)) {
                reached.insert(i);
                if !closed.contains(&i) {
                    open.push(i);
                    came_from.insert(i, current);
                }
            }
        }
    }

    println!("{:?} {:?}", reached.len(), scanners.len());
    if reached.len() != scanners.len() {
        panic!("not all reached");
    }

    let mut scanner_positions = HashMap::new();
    for i in 0..scanners.len() {
        let mut position = Coord { x: 0, y: 0, z: 0 };
        let mut current = i;
        loop {
            if let Some(prev) = came_from.get(&current) {
                let (matrix, offset) = transforms[&(current, *prev)];
                position = position.apply_matrix(&matrix);
                position.x += offset.x;
                position.y += offset.y;
                position.z += offset.z;
                current = *prev;
                if current == 0 {
                    break;
                }
            } else {
                break;
            }
        }
        assert_eq!(current, 0);
        scanner_positions.insert(i, position);
    }
    println!("scanner_positions = {:?}", scanner_positions);
    println!("scanner_positions.len() = {}", scanner_positions.len());

    let mut manhattan_distances = HashMap::new();
    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            if j <= i {
                continue;
            }
            let a = scanner_positions[&i];
            let b = scanner_positions[&j];
            let manhattan_distance = (b.x - a.x).abs() + (b.y - a.y).abs() + (b.z - a.z).abs();
            manhattan_distances.insert((i, j), manhattan_distance);
            manhattan_distances.insert((j, i), manhattan_distance);
        }
    }
    println!("manhattan_distances = {:?}", manhattan_distances);
    let max_manhattan_distance = manhattan_distances.iter().max_by(|(_, v), (_, v2)| v.cmp(v2));
    println!("max_manhattan_distance = {:?}", max_manhattan_distance);

    return 0;
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_matrices() {
        for matrix in MATRICES {
            let v = Coord { x: 1, y: 1, z: 1 };
            assert_eq!(v.apply_matrix(matrix), v);
        }
    }
}