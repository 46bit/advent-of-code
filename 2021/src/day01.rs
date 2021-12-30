use itertools::Itertools;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut depths = input.lines().map(|s| s.parse::<i32>().unwrap());
    let mut previous: i32 = depths.next().unwrap();
    let mut increases = 0;
    for depth in depths {
        if depth > previous {
            increases += 1;
        }
        previous = depth;
    }
    return increases;
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let depths = input.lines().map(|s| s.parse::<i32>().unwrap());
    let depth_windows = depths.tuple_windows::<(_, _, _)>();
    let mut sums = depth_windows.map(|(a, b, c)| a + b + c).peekable();
    let mut increases = 0;
    while let (Some(previous), Some(next)) = (sums.next(), sums.peek()) {
        if *next > previous {
            increases += 1;
        }
    }
    return increases;
}

#[cfg(test)]
mod tests {
    pub use super::*;

    const INPUT: &str = include_str!("../input/2021/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1342);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1378);
    }
}
