use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Scanner {
    id: String,
    beacons: Vec<(i64, i64, i64)>,
}

#[aoc(day19, part1)]
fn part1(input: &str) -> i32 {
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
            beacons.push((x, y, z));
        }
        scanners.push(Scanner{id, beacons});
    }
    println!("{:?}", scanners);

    let mut beacon_distances = HashMap::new();
    for scanner in &scanners {
        let beacon_count = scanner.beacons.len();
        for i in 0..beacon_count {
            for j in 0..beacon_count {
                if i == j {
                    continue;
                }
                let a = scanner.beacons[i].clone();
                let b = scanner.beacons[j].clone();
                let distance = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2);
                *beacon_distances.entry(distance).or_insert(0) += 1;
            }
        }
    }
    println!("{:?}", beacon_distances);
    let mut summary = HashMap::new();
    for (_, n) in &beacon_distances {
        *summary.entry(*n).or_insert(0) += 1;
    }
    println!("{:?}", summary);

    let mut useful_beacon_count = 0;
    let mut useful_beacon_links = 0;
    for scanner in &scanners {
        let beacon_count = scanner.beacons.len();
        for i in 0..beacon_count {
            let mut useful = false;
            for j in 0..beacon_count {
                if i == j {
                    continue;
                }
                let a = scanner.beacons[i].clone();
                let b = scanner.beacons[j].clone();
                let distance = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2);
                if beacon_distances[&distance] > 2 {
                    useful = true;
                    useful_beacon_links += 1;
                }
            }
            if useful {
                useful_beacon_count += 1;
            }
        }
    }
    println!("{:?}", useful_beacon_count);
    println!("{:?}", useful_beacon_links);

    let mut notables: HashMap<i64, (String, (i64, i64, i64), (i64, i64, i64))> = HashMap::new();
    let mut unique_distance_pairs = vec![];
    for scanner in &scanners {
        let beacon_count = scanner.beacons.len();
        for i in 0..beacon_count {
            for j in 0..beacon_count {
                if i == j {
                    continue;
                }
                let a = scanner.beacons[i].clone();
                let b = scanner.beacons[j].clone();
                let distance = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2);
                if beacon_distances[&distance] == 4 {
                    if notables.contains_key(&distance) && notables[&distance].0 != scanner.id {
                        let (other_scanner_id, s1, s2) = notables.remove(&distance).unwrap();
                        if other_scanner_id != scanner.id {
                            unique_distance_pairs.push(((other_scanner_id, s1, s2), (scanner.id.clone(), a, b)));
                        }
                    } else {
                        notables.insert(distance, (scanner.id.clone(), a, b));
                    }
                }
            }
        }
    }
    println!("{:?}", unique_distance_pairs);
    let mut joins: HashMap<String, HashSet<String>> = HashMap::new();
    for scanner in &scanners {
        joins.insert(scanner.id.clone(), HashSet::new());
    }
    for unique_distance_pair in unique_distance_pairs {
        joins.get_mut(&unique_distance_pair.0.0).unwrap().insert(unique_distance_pair.1.0.clone());
        joins.get_mut(&unique_distance_pair.1.0).unwrap().insert(unique_distance_pair.0.0.clone());
    }
    println!("{:?}", joins);

    //let globally_joined = true;
    // for scanner in &scanners {
    //     // let search = |what: String| {
    //     //     joins[what]
    //     // }
    // }

    return 0;
}

//fn 

// #[aoc(day19, part2)]
// fn part2(input: &str) -> i32 {
//     return 0;
// }
