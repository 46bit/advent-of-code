use regex::Regex;
use std::fmt::Debug;
use std::collections::HashMap;

trait VoxelSpace {
    type Voxel;

    fn get(&self, x: i64, y: i64, z: i64) -> Self::Voxel;

    fn act(
        &mut self, 
        xs: impl Iterator<Item = i64>, 
        ys: impl Iterator<Item = i64>, 
        zs: impl Iterator<Item = i64>, 
        action: impl FnMut(&mut Self::Voxel),
    );
}

// #[derive(Clone, Debug)]
// struct FiniteVoxelSpace<const SIZE: usize, VOXEL: Clone + Debug> {
//     offset: i64,
//     xyz_voxels: [[[VOXEL; SIZE]; SIZE]; SIZE],
// }

// impl<const SIZE: usize, VOXEL: Clone + Debug> VoxelSpace for FiniteVoxelSpace<SIZE, VOXEL> {
//     type Voxel = VOXEL;

//     fn get(&self, x: i64, y: i64, z: i64) -> Self::Voxel {
//         println!("{} {} {}", x, y, z);
//         let adjusted_x = x - self.offset;
//         let adjusted_y = y - self.offset;
//         let adjusted_z = z - self.offset;
//         println!("{} {} {}", adjusted_x, adjusted_y, adjusted_z);
//         self.xyz_voxels[adjusted_x as usize][adjusted_y as usize][adjusted_z as usize].clone()
//     }

//     fn act(
//         &mut self, 
//         mut xs: impl Iterator<Item = i64>, 
//         mut ys: impl Iterator<Item = i64>, 
//         mut zs: impl Iterator<Item = i64>, 
//         mut action: impl FnMut(&mut Self::Voxel),
//     ) {
//         let max = SIZE as i64;
//         for x in &mut xs {
//             let adjusted_x = x - self.offset;
//             for y in &mut ys {
//                 let adjusted_y = y - self.offset;
//                 for z in &mut zs {
//                     println!("{} {} {}", x, y, z);
//                     let adjusted_z = z - self.offset;
//                     if adjusted_x < 0 || adjusted_x > max {
//                         continue;
//                     }
//                     if adjusted_y < 0 || adjusted_y > max {
//                         continue;
//                     }
//                     if adjusted_z < 0 || adjusted_z > max {
//                         continue;
//                     }
//                     println!("adjusted: {} {} {}", adjusted_x, adjusted_y, adjusted_z);
//                     action(&mut self.xyz_voxels[adjusted_x as usize][adjusted_y as usize][adjusted_z as usize]);
//                 }
//             }
//         }
//     }
// }

#[derive(Clone, Debug)]
struct InfiniteVoxelSpace<VOXEL: Clone + Debug + Default> {
    xyz_voxels: HashMap<(i64, i64, i64), VOXEL>,
}

impl<VOXEL: Clone + Debug + Default> VoxelSpace for InfiniteVoxelSpace<VOXEL> {
    type Voxel = VOXEL;

    fn get(&self, x: i64, y: i64, z: i64) -> Self::Voxel {
        if self.xyz_voxels.contains_key(&(x, y, z)) {
            self.xyz_voxels[&(x, y, z)].clone()
        } else {
            Self::Voxel::default()
        }
    }

    fn act(
        &mut self, 
        mut xs: impl Iterator<Item = i64>, 
        mut ys: impl Iterator<Item = i64>, 
        mut zs: impl Iterator<Item = i64>, 
        mut action: impl FnMut(&mut Self::Voxel),
    ) {
        for x in &mut xs {
            for y in &mut ys {
                for z in &mut zs {
                    let mut entry = self.xyz_voxels.entry((x, y, z)).or_insert(Self::Voxel::default());
                    action(&mut entry);
                }
            }
        }
    }
}

#[aoc(day22, part1)]
fn part1(input: &str) -> u64 {
    // let mut voxel_space = InfiniteVoxelSpace{
    //     xyz_voxels: HashMap::new(),
    // };

    let mut voxels = HashMap::new();

    let re = Regex::new(r"(?P<state>[a-z]+) x=(?P<x1>[-0-9]+)..(?P<x2>[-0-9]+),y=(?P<y1>[-0-9]+)..(?P<y2>[-0-9]+),z=(?P<z1>[-0-9]+)..(?P<z2>[-0-9]+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let state = match caps.name("state").unwrap().as_str() {
            "on" => true,
            "off" => false,
            _ => panic!("input unparseable"),
        };
        //let f = |s: &mut bool| *s = state;

        let x1 = caps.name("x1").unwrap().as_str().parse().unwrap();
        let x2 = caps.name("x2").unwrap().as_str().parse().unwrap();
        let xs = x1..=x2;
        let y1 = caps.name("y1").unwrap().as_str().parse().unwrap();
        let y2 = caps.name("y2").unwrap().as_str().parse().unwrap();
        let ys = y1..=y2;
        let z1 = caps.name("z1").unwrap().as_str().parse().unwrap();
        let z2 = caps.name("z2").unwrap().as_str().parse().unwrap();
        let zs = z1..=z2;

        //voxel_space.act(xs, ys, zs, f);
        for x in xs.clone() {
            if x < -50 || x > 50 {
                continue;
            }
            for y in ys.clone() {
                if y < -50 || y > 50 {
                    continue;
                }
                for z in zs.clone() {
                    if z < -50 || z > 50 {
                        continue;
                    }
                    voxels.insert((x, y, z), state);
                }
            }
        }
    }

    let mut number_of_cubes_on = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                //if voxel_space.get(x, y, z) {
                let key = (x, y, z);
                if voxels.contains_key(&key) && voxels[&key] == true {
                    number_of_cubes_on += 1;
                }
            }
        }
    }
    return number_of_cubes_on;
}

#[derive(Clone, Debug)]
struct Square {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
    rear: i64,
    fore: i64,
}

impl Square {
    fn intersects(&self, other: &Square) -> bool {
        if other.right < self.left || self.right < other.left {
            return false;
        }
        if other.bottom < self.top || self.bottom < other.top {
            return false;
        }
        if other.fore < self.rear || self.fore < other.rear {
            return false;
        }
        true
    }

    // fn empty(&self) -> bool {
    //     self.left == self.right || self.top == self.bottom
    // }

    fn squares(&self) -> Vec<(i64, i64, i64)> {
        let mut s = vec![];
        for x in self.left..=self.right {
            for y in self.top..=self.bottom {
                for z in self.rear..=self.fore {
                    s.push((x, y, z));
                }
            }
        }
        return s;
    }

    fn squares_count(&self) -> u64 {
        let xn = (self.right - self.left + 1).abs() as u64;
        let yn = (self.bottom - self.top + 1).abs() as u64;
        let zn = (self.fore - self.rear + 1).abs() as u64;
        xn * yn * zn
    }

    fn subtract(&self, mut other: Square) -> Vec<Square> {
        if !self.intersects(&other) {
            return vec![self.clone()];
        }

        if other.left < self.left {
            other.left = self.left;
        }
        if other.top < self.top {
            other.top = self.top;
        }
        if other.rear < self.rear {
            other.rear = self.rear;
        }
        if other.right > self.right {
            other.right = self.right;
        }
        if other.bottom > self.bottom {
            other.bottom = self.bottom;
        }
        if other.fore > self.fore {
            other.fore = self.fore;
        }
        
        let mut squares = vec![];
        // LEFT SQUARE
        if other.left > self.left {
            squares.push(Square{
                left: self.left,
                right: other.left - 1,
                top: self.top,
                bottom: self.bottom,
                rear: self.rear,
                fore: self.fore,
            });
        }
        // RIGHT SQUARE
        if self.right > other.right {
            squares.push(Square{
                left: other.right + 1,
                right: self.right,
                top: self.top,
                bottom: self.bottom,
                rear: self.rear,
                fore: self.fore,
            });
        }
        // TOP MIDDLE
        if other.top > self.top {
            squares.push(Square{
                left: other.left,
                right: other.right,
                top: self.top,
                bottom: other.top - 1,
                rear: self.rear,
                fore: self.fore,
            });
        }
        // BOTTOM MIDDLE
        if self.bottom > other.bottom {
            squares.push(Square{
                left: other.left,
                right: other.right,
                top: other.bottom + 1,
                bottom: self.bottom,
                rear: self.rear,
                fore: self.fore,
            });
        }
        // REAR MIDDLE
        if other.rear > self.rear {
            squares.push(Square{
                left: other.left,
                right: other.right,
                top: other.top,
                bottom: other.bottom,
                rear: self.rear,
                fore: other.rear - 1,
            });
        }
        // FORE MIDDLE
        if self.fore > other.fore {
            squares.push(Square{
                left: other.left,
                right: other.right,
                top: other.top,
                bottom: other.bottom,
                rear: other.fore + 1,
                fore: self.fore,
            });
        }
        return squares;//.into_iter().filter(Square::empty).collect();
    }
}

#[aoc(day22, part2)]
fn part2(input: &str) -> u64 {
    let s1 = Square{
        left: -5,
        right: 5,
        top: -5,
        bottom: 5,
        rear: 0,
        fore: 0,
    };
    assert_eq!(s1.squares().len(), 121);
    let s2 = Square{
        left: -2,
        right: 2,
        top: -2,
        bottom: 2,
        rear: 0,
        fore: 0,
    };
    assert_eq!(s2.squares().len(), 25);
    let s1s = s1.subtract(s2);
    assert_eq!(s1s.clone().into_iter().map(|s| s.squares().len()).sum::<usize>(), 121 - 25);
    // for s in s1s {
    //     println!("1. {:?}", s);
    //     println!("2. {:?}", s.squares());
    // }

    let mut squares = vec![];

    let re = Regex::new(r"(?P<state>[a-z]+) x=(?P<x1>[-0-9]+)..(?P<x2>[-0-9]+),y=(?P<y1>[-0-9]+)..(?P<y2>[-0-9]+),z=(?P<z1>[-0-9]+)..(?P<z2>[-0-9]+)").unwrap();
    for line in input.lines() {
        println!("squares={} {}", squares.len(), line);
        let caps = re.captures(line).unwrap();

        let state = match caps.name("state").unwrap().as_str() {
            "on" => true,
            "off" => false,
            _ => panic!("input unparseable"),
        };

        let x1 = caps.name("x1").unwrap().as_str().parse().unwrap();
        let x2 = caps.name("x2").unwrap().as_str().parse().unwrap();
        let y1 = caps.name("y1").unwrap().as_str().parse().unwrap();
        let y2 = caps.name("y2").unwrap().as_str().parse().unwrap();
        let z1 = caps.name("z1").unwrap().as_str().parse().unwrap();
        let z2 = caps.name("z2").unwrap().as_str().parse().unwrap();
        let square = Square{
            left: x1,
            right: x2,
            top: y1,
            bottom: y2,
            rear: z1,
            fore: z2,
        };

        if state {
            squares.push(square);
        } else {
            squares = squares.into_iter().map(|s| s.subtract(square.clone())).flatten().collect();
        }
    }

    for s in squares.clone() {
        println!("1. {:?}", s);
        //println!("   {:?}", s.squares());
    }

    // deduplicate square overlap
    // let mut cont = true;
    // while cont {
    //     for r1 in squares.clone() {
    //         for r2 in squares.clone() {
    //             if r1 == r2 {
    //                 squares = squares.into_iter().concat(r1.subtract(r2).into_iter()).collect();
    //             }
    //         }
    //     }
    // }

    let mut n = 0;
    for (i, square) in squares.clone().into_iter().enumerate() {
        let mut s3 = vec![square];
        for (j, r2) in squares.clone().into_iter().enumerate() {
            if i <= j {
                continue;
            }
            s3 = s3.into_iter().map(|r| r.subtract(r2.clone())).flatten().collect();
        }
        for z in s3 {
            n += z.squares_count();
        }
    }
    return n;
    //squares[0].subtract(squares[1])
    //return squares.into_iter().map(|s| s.squares_count()).sum();
}