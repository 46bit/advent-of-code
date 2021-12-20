use std::collections::HashSet;

#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    let entries: Vec<_> = input
        .lines()
        .map(|line| {
            let tokens: Vec<_> = line.split(" | ").collect();
            let inputs: Vec<_> = tokens[0]
                .split(" ")
                .map(|s| {
                    let mut chars: HashSet<_> = s.chars().collect();
                    chars
                })
                .collect();
            let outputs: Vec<_> = tokens[1]
                .split(" ")
                .map(|s| {
                    let mut chars: HashSet<_> = s.chars().collect();
                    chars
                })
                .collect();
            [inputs, outputs]
        })
        .collect();

    // let mut one = None;
    // let mut four = None;
    // let mut seven = None;
    // let mut eight = None;
    let mut count = 0;
    for entry in &entries {
        for token in &entry[1] {
            match token.len() {
                2 | 3 | 4 | 7 => {
                    count += 1;
                }
                _ => {}
            }
        }
    }

    return count;
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    let entries: Vec<_> = input
        .lines()
        .map(|line| {
            let tokens: Vec<_> = line.split(" | ").collect();
            let inputs: Vec<_> = tokens[0]
                .split(" ")
                .map(|s| {
                    let mut chars: HashSet<_> = s.chars().collect();
                    chars
                })
                .collect();
            let outputs: Vec<_> = tokens[1]
                .split(" ")
                .map(|s| {
                    let mut chars: HashSet<_> = s.chars().collect();
                    chars
                })
                .collect();
            [inputs, outputs]
        })
        .collect();

    for entry in &entries {
        let mut mappings: Vec<Option<HashSet<char>>> = vec![None; 10];
        for (i, token) in entry[0].iter().enumerate() {
            match token.len() {
                2 => mappings[1] = Some(token.clone()),
                3 => mappings[7] = Some(token.clone()),
                4 => mappings[4] = Some(token.clone()),
                7 => mappings[8] = Some(token.clone()),
                _ => {}
            }
        }
        println!("{:?}", mappings);
        // check all 4 unique digits are supplied
        assert_eq!(
            mappings
                .iter()
                .filter(|m| m.is_some())
                .collect::<Vec<_>>()
                .len(),
            4
        );
        // We now have 1, 4, 7 and 8
        // 1 and 7 differ only in that 7 has A
        let candidates: Vec<_> = mappings[7]
            .as_ref()
            .unwrap()
            .difference(&mappings[1].as_ref().unwrap())
            .collect();
        assert_eq!(candidates.len(), 1);
        let a = candidates[0];
        // B is in 0 but

        // // The only segment shared by 1 and 2 is C
        // let c = intersect(mappings[1].clone().unwrap(), mappings[2].clone().unwrap())[0];
        // // The only other segment shared by 2 and 7 is A
        // let a_candidates = intersect(mappings[2].clone().unwrap(), mappings[7].clone().unwrap());
        // let a = a_candidates.into_iter().filter(|s| *s != c).next().unwrap();
        // // The only other segment shared by 1 and 7 is F
        // let f_candidates = overlap(mappings[1].clone().unwrap(), mappings[7].clone().unwrap());
        // println!("{:?}", f_candidates);
        // let f = f_candidates
        //     .into_iter()
        //     .filter(|s| *s != a && *s != c)
        //     .collect::<Vec<_>>()[0];
        // println!("{} {} {}", c, a, f);
    }

    return 0;
}

fn intersect(a: Vec<char>, b: Vec<char>) -> Vec<char> {
    let mut overlap = vec![];
    for i in a {
        for j in &b {
            if i == *j {
                if !overlap.contains(&i) {
                    overlap.push(i);
                }
            }
        }
    }
    overlap
}

fn overlap(a: Vec<char>, b: Vec<char>) -> Vec<char> {
    let mut overlap = vec![];
    for i in a {
        if !overlap.contains(&i) {
            overlap.push(i);
        }
    }
    for j in b {
        if !overlap.contains(&j) {
            overlap.push(j);
        }
    }
    overlap
}
