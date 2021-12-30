use regex::Regex;

// FIXME: Analyse problem properly

#[aoc(day17, part1)]
pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"x=(?P<x1>[-0-9]+)..(?P<x2>[-0-9]+).*y=(?P<y1>[-0-9]+)..(?P<y2>[-0-9]+)")
        .unwrap();
    let caps = re.captures(input).unwrap();
    let x1: i64 = caps.name("x1").unwrap().as_str().parse().unwrap();
    let x2: i64 = caps.name("x2").unwrap().as_str().parse().unwrap();
    let y1: i64 = caps.name("y1").unwrap().as_str().parse().unwrap();
    let y2: i64 = caps.name("y2").unwrap().as_str().parse().unwrap();
    println!("x1={} x2={} y1={} y2={}", x1, x2, y1, y2);

    let mut overall_max_y = i64::MIN;
    let mut best_v = None;
    for vy in -1000..=1000 {
        for vx in -1000..=1000 {
            let (success, max_y) = simulate(vy, vx, x1, x2, y1, y2);
            if !success {
                continue;
            }
            //println!("v=({}, {}) success={} max_y={}", vy, vx, success, max_y);
            if max_y > overall_max_y {
                overall_max_y = max_y;
                best_v = Some((vy, vx));
            }
        }
    }
    println!("best: v=({:?}) max_y={}", best_v, overall_max_y);

    return overall_max_y;
}

fn simulate(mut vy: i64, mut vx: i64, x1: i64, x2: i64, y1: i64, y2: i64) -> (bool, i64) {
    let mut y = 0;
    let mut x = 0;
    let mut max_y = i64::MIN;

    let mut xs = vec![x1, x2];
    xs.sort();
    let xr = xs[0]..=xs[1];
    let mut ys = vec![y1, y2];
    ys.sort();
    let yr = ys[0]..=ys[1];

    while y >= ys[1] {
        y += vy;
        x += vx;
        if y > max_y {
            max_y = y;
        }

        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }
        vy -= 1;

        if xr.contains(&x) && yr.contains(&y) {
            return (true, max_y);
        }

        // if (y2 > 0 && y > y2) || (y2 < 0 && y < y2) || (x2 > 0 && x > x2) || (x2 < 0 && x < x2) {
        //     return (false, max_y);
        // }

        // if (y1 > 0 && y > y1) || (y1 < 0 && y < y1) || (x1 > 0 && x > x1) || (x1 < 0 && x < x1) {
        //     return (true, max_y);
        // }
    }
    return (false, max_y);
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    let re = Regex::new(r"x=(?P<x1>[-0-9]+)..(?P<x2>[-0-9]+).*y=(?P<y1>[-0-9]+)..(?P<y2>[-0-9]+)")
        .unwrap();
    let caps = re.captures(input).unwrap();
    let x1: i64 = caps.name("x1").unwrap().as_str().parse().unwrap();
    let x2: i64 = caps.name("x2").unwrap().as_str().parse().unwrap();
    let y1: i64 = caps.name("y1").unwrap().as_str().parse().unwrap();
    let y2: i64 = caps.name("y2").unwrap().as_str().parse().unwrap();
    println!("x1={} x2={} y1={} y2={}", x1, x2, y1, y2);

    let mut uniques = 0;
    for vy in -1000..=1000 {
        for vx in -1000..=1000 {
            let (success, _) = simulate(vy, vx, x1, x2, y1, y2);
            if success {
                uniques += 1;
            }
        }
    }

    return uniques;
}

#[cfg(test)]
mod tests {
    pub use super::*;

    const INPUT: &str = include_str!("../input/2021/day17.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35511);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3282);
    }
}