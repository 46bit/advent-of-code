#[aoc(day7, part1)]
fn part1(input: &str) -> i64 {
    let mut positions: Vec<i64> = input
        .split(",")
        .map(|t| t.parse())
        .map(Result::unwrap)
        .collect();

    positions.sort();
    let median = positions[positions.len() / 2] as i64;
    println!("{}", median);
    let fuel_cost = positions
        .into_iter()
        .map(|p| (median - p).abs())
        .sum::<i64>();
    return fuel_cost;
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i64 {
    let positions: Vec<i64> = input
        .split(",")
        .map(|t| t.parse())
        .map(Result::unwrap)
        .collect();
    let mut costs = [0; 2000];
    for position in positions {
        for i in 0..2000 {
            let distance = (i as i64 - position).abs();
            costs[i] += distance * (distance + 1) / 2;
        }
    }
    let mut min_index = 0;
    let mut min_value = i64::MAX;
    for i in 0..2000 {
        if costs[i] < min_value {
            min_value = costs[i];
            min_index = i;
        }
    }
    return costs[min_index];
}
