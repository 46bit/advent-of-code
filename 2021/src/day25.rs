use core::cmp::max;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct SeaCucumber {
    y: usize,
    x: usize,
    vy: usize,
    vx: usize,
}

impl SeaCucumber {
    fn next_location(&self, height: usize, width: usize) -> (usize, usize) {
        let y2 = (self.y + self.vy) % height;
        let x2 = (self.x + self.vx) % width;
        (y2, x2)
    }
}

const HEIGHT: usize = 137;
const WIDTH: usize = 139;

#[aoc(day25, part1)]
fn part1(input: &str) -> i32 {
    let mut height = 0;
    let mut width = 0;
    let mut sea_cucumbers = vec![];
    for (y, line) in input.lines().enumerate() {
        height = max(height, y+1);
        for (x, char_) in line.chars().enumerate() {
            width = max(width, x+1);
            let (vy, vx) = match char_ {
                '>' => (0, 1),
                'v' => (1, 0),
                '.' => continue,
                _ => unreachable!(),
            };
            //herds.entry((vy, vx)).or_default().push(sea_cucumber);
            let sea_cucumber = SeaCucumber{y, x, vy, vx};
            sea_cucumbers.push(sea_cucumber);
        }
    }
    assert_eq!(height, HEIGHT);
    assert_eq!(width, WIDTH);

    let mut yx_sea_cucumbers = BTreeMap::new();
    for sea_cucumber in sea_cucumbers {
        yx_sea_cucumbers.insert((sea_cucumber.y, sea_cucumber.x), sea_cucumber);
    }

    println!("--------");
    println!("ORIGINAL:");
    print_sea_cucumbers(&yx_sea_cucumbers, height, width);
    println!("--------");

    for i in 1.. {
        let mut moved = false;
        for (vy, vx) in [(0, 1), (1, 0)] {
            let old_yx_sea_cucumbers = yx_sea_cucumbers.clone();
            for (current_location, sea_cucumber) in yx_sea_cucumbers.clone() {
                if (sea_cucumber.vy, sea_cucumber.vx) != (vy, vx) {
                    continue;
                }
                let new_location = sea_cucumber.next_location(HEIGHT, WIDTH);
                if !old_yx_sea_cucumbers.contains_key(&new_location) {
                    moved = true;
                    let mut new_sea_cucumber = sea_cucumber.clone();
                    new_sea_cucumber.y = new_location.0;
                    new_sea_cucumber.x = new_location.1;
                    yx_sea_cucumbers.insert(new_location, new_sea_cucumber);
                    yx_sea_cucumbers.remove(&current_location);
                }
            }
        }

        println!("#{}", i);
        print_sea_cucumbers(&yx_sea_cucumbers, height, width);

        if !moved {
            println!("NOTHING MOVED");
            return i;
        }

        println!("--------");
    }

    unreachable!();
}

fn print_sea_cucumbers(yx_sea_cucumbers: &BTreeMap<(usize, usize), SeaCucumber>, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", match yx_sea_cucumbers.get(&(y, x)) {
                None => '.',
                Some(sea_cucumber) => match (sea_cucumber.vy, sea_cucumber.vx) {
                    (1, 0) => 'v',
                    (0, 1) => '>',
                    _ => unimplemented!(),
                }
            });
        }
        println!();
    }
}

// #[aoc(day25, part2)]
// fn part2(input: &str) -> i32 {
//     return 0;
// }
