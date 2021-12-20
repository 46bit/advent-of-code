use std::collections::{HashMap, HashSet};

#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    let path_strings: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.split("-").collect::<Vec<_>>())
        .collect();
    let mut paths = HashMap::new();
    for path_string in &path_strings {
        paths.insert(path_string[0], HashSet::new());
        paths.insert(path_string[1], HashSet::new());
    }
    for path_string in path_strings {
        paths
            .get_mut(&path_string[0])
            .unwrap()
            .insert(path_string[1]);
        paths
            .get_mut(&path_string[1])
            .unwrap()
            .insert(path_string[0]);
    }
    //println!("{:?}", paths);

    let mut routes = HashSet::new();
    let mut queued = HashSet::new();
    let mut unfinisheds = Vec::new();
    queued.insert(vec!["start"]);
    unfinisheds.push(vec!["start"]);
    while let Some(unfinished) = unfinisheds.pop() {
        let nexts = paths.get(&unfinished[unfinished.len() - 1]).unwrap();
        for next in nexts {
            let mut next_route = unfinished.clone();
            next_route.push(next);
            if next.clone().chars().next().unwrap().is_ascii_lowercase() {
                if unfinished.contains(next) {
                    continue;
                }
            }
            if *next == "end" {
                routes.insert(next_route);
                continue;
            }
            if *next == "start" {
                continue;
            }
            if !queued.contains(&next_route) {
                queued.insert(next_route.clone());
                unfinisheds.push(next_route);
            }
        }
    }
    //println!("{:?}", routes);
    //println!("{:?}", routes.len());

    let mut routes_visiting_small_caves = HashSet::new();
    for route in &routes {
        for cave in &route[1..route.len() - 1] {
            if cave.clone().chars().next().unwrap().is_ascii_lowercase() {
                routes_visiting_small_caves.insert(route);
                break;
            }
        }
    }

    return routes_visiting_small_caves.len() as i32;
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    let path_strings: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.split("-").collect::<Vec<_>>())
        .collect();
    let mut paths = HashMap::new();
    for path_string in &path_strings {
        paths.insert(path_string[0], HashSet::new());
        paths.insert(path_string[1], HashSet::new());
    }
    for path_string in path_strings {
        paths
            .get_mut(&path_string[0])
            .unwrap()
            .insert(path_string[1]);
        paths
            .get_mut(&path_string[1])
            .unwrap()
            .insert(path_string[0]);
    }
    //println!("{:?}", paths);

    let mut routes = HashSet::new();
    let mut queued = HashSet::new();
    let mut unfinisheds = Vec::new();
    queued.insert(vec!["start"]);
    unfinisheds.push(vec!["start"]);
    while let Some(unfinished) = unfinisheds.pop() {
        let nexts = paths.get(&unfinished[unfinished.len() - 1]).unwrap();
        for next in nexts {
            let mut next_route = unfinished.clone();
            next_route.push(next);
            if !has_no_more_than_one_doubled_lowercase_cave(next_route.clone()) {
                continue;
            }
            if *next == "end" {
                routes.insert(next_route);
                continue;
            }
            if *next == "start" {
                continue;
            }
            if !queued.contains(&next_route) {
                queued.insert(next_route.clone());
                unfinisheds.push(next_route);
            }
        }
    }
    //println!("{:?}", routes);
    //println!("{:?}", routes.len());

    let mut routes_visiting_small_caves = HashSet::new();
    for route in &routes {
        for cave in &route[1..route.len() - 1] {
            if cave.clone().chars().next().unwrap().is_ascii_lowercase() {
                routes_visiting_small_caves.insert(route);
                break;
            }
        }
    }

    return routes_visiting_small_caves.len() as i32;
}

fn has_no_more_than_one_doubled_lowercase_cave(route: Vec<&str>) -> bool {
    let mut lowercase = HashSet::new();
    let mut repeated_one = false;
    for cave in route {
        if !cave.clone().chars().next().unwrap().is_ascii_lowercase() {
            continue;
        }
        if lowercase.contains(&cave) {
            if repeated_one {
                return false;
            }
            repeated_one = true;
        } else {
            lowercase.insert(cave);
        }
    }
    return true;
}
