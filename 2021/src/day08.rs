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
                    let chars: HashSet<_> = s.chars().collect();
                    chars
                })
                .collect();
            let outputs: Vec<_> = tokens[1]
                .split(" ")
                .map(|s| {
                    let chars: HashSet<_> = s.chars().collect();
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
fn part2(input: &str) -> i64 {
    let entries: Vec<_> = input
        .lines()
        .map(|line| {
            let tokens: Vec<_> = line.split(" | ").collect();
            let inputs: Vec<_> = tokens[0]
                .split(" ")
                .map(|s| {
                    let chars: HashSet<_> = s.chars().collect();
                    chars
                })
                .collect();
            let outputs: Vec<_> = tokens[1]
                .split(" ")
                .map(|s| {
                    let chars: HashSet<_> = s.chars().collect();
                    chars
                })
                .collect();
            [inputs, outputs]
        })
        .collect();

    let mut outsum = 0;

    for entry in &entries {
        let mut mappings: Vec<Option<HashSet<char>>> = vec![None; 10];
        for token in entry[0].iter() {
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
        println!("a: {:?}", a);

        // F is in 1 and used by all 3 of the six-segment digits
        let six_segment_digits: Vec<_> = entry[0].iter().filter(|t| t.len() == 6).collect();
        assert_eq!(six_segment_digits.len(), 3);
        let f_and_c: Vec<_> = mappings[1].clone().unwrap().into_iter().collect();
        assert_eq!(f_and_c.len(), 2);
        let mut c = None;
        let mut f = None;
        for six_segment_digit in six_segment_digits {
            if !six_segment_digit.contains(&f_and_c[0]) {
                c = Some(f_and_c[0]);
                f = Some(f_and_c[1]);
                break;
            }
        }
        if c.is_none() {
            c = Some(f_and_c[1]);
            f = Some(f_and_c[0]);
        }
        assert!(c.is_some());
        assert!(f.is_some());
        println!("c: {:?}", c);
        println!("f: {:?}", f);

        // B and D are in 4, but only D is used by all the five-segment digits
        let five_segment_digits: Vec<_> = entry[0].iter().filter(|t| t.len() == 5).collect();
        assert_eq!(five_segment_digits.len(), 3);
        let b_and_d: Vec<_> = mappings[4].clone().unwrap().into_iter().filter(|v| *v != c.unwrap() && *v != f.unwrap()).collect();
        assert_eq!(b_and_d.len(), 2);
        let mut b = None;
        let mut d = None;
        for five_segment_digit in &five_segment_digits {
            if !five_segment_digit.contains(&b_and_d[0]) {
                b = Some(b_and_d[0]);
                d = Some(b_and_d[1]);
                break;
            }
        }
        if b.is_none() {
            b = Some(b_and_d[1]);
            d = Some(b_and_d[0]);
        }
        assert!(b.is_some());
        assert!(d.is_some());
        println!("b: {:?}", b);
        println!("d: {:?}", d);

        // G and E are in 8, but only G is used by all the five-segment digits
        let g_and_e: Vec<_> = mappings[8].clone().unwrap().into_iter().filter(|v| {
            *v != c.unwrap() && *v != f.unwrap() && v != a && *v != b.unwrap() && *v != d.unwrap()
        }).collect();
        assert_eq!(g_and_e.len(), 2);
        let mut e = None;
        let mut g = None;
        for five_segment_digit in &five_segment_digits {
            if !five_segment_digit.contains(&g_and_e[0]) {
                e = Some(g_and_e[0]);
                g = Some(g_and_e[1]);
                break;
            }
        }
        if e.is_none() {
            e = Some(g_and_e[1]);
            g = Some(g_and_e[0]);
        }
        assert!(e.is_some());
        assert!(g.is_some());
        println!("e: {:?}", e);
        println!("g: {:?}", g);

        let mut outs = vec![
            format!("{}{}{}{}{}{}", a, b.unwrap(), c.unwrap(), e.unwrap(), f.unwrap(), g.unwrap()),
            format!("{}{}", c.unwrap(), f.unwrap()),
            format!("{}{}{}{}{}", a, c.unwrap(), d.unwrap(), e.unwrap(), g.unwrap()),
            format!("{}{}{}{}{}", a, c.unwrap(), d.unwrap(), f.unwrap(), g.unwrap()),
            format!("{}{}{}{}", b.unwrap(), c.unwrap(), d.unwrap(), f.unwrap()),
            format!("{}{}{}{}{}", a, b.unwrap(), d.unwrap(), f.unwrap(), g.unwrap()),
            format!("{}{}{}{}{}{}", a, b.unwrap(), d.unwrap(), e.unwrap(), f.unwrap(), g.unwrap()),
            format!("{}{}{}", a, c.unwrap(), f.unwrap()),
            format!("{}{}{}{}{}{}{}", a, b.unwrap(), c.unwrap(), d.unwrap(), e.unwrap(), f.unwrap(), g.unwrap()),
            format!("{}{}{}{}{}{}", a, b.unwrap(), c.unwrap(), d.unwrap(), f.unwrap(), g.unwrap()),
        ];
        for out in &mut outs {
            assert!(entry[0].contains(&out.clone().chars().collect()));
            let mut chars: Vec<_> = out.clone().chars().collect();
            chars.sort();
            *out = chars.into_iter().collect();
        }
        println!("{:?}", outs);

        let mut sum = 0;
        for s in &entry[1] {
            let mut chars: Vec<_> = s.clone().into_iter().collect();
            chars.sort();
            let sorted_s: String = chars.into_iter().collect();

            let mut found = None;
            for (i, v) in outs.iter().enumerate() {
                if *v == sorted_s {
                    println!("{:?}", sorted_s);
                    found = Some(i);
                }
            }
            if found.is_none() {
                panic!("fail");
            }
            println!("{}", found.unwrap());
            sum = sum * 10 + (found.unwrap() as i64);
        }
        outsum += sum;
    }

    return outsum;
}

// fn intersect(a: Vec<char>, b: Vec<char>) -> Vec<char> {
//     let mut overlap = vec![];
//     for i in a {
//         for j in &b {
//             if i == *j {
//                 if !overlap.contains(&i) {
//                     overlap.push(i);
//                 }
//             }
//         }
//     }
//     overlap
// }

// fn overlap(a: Vec<char>, b: Vec<char>) -> Vec<char> {
//     let mut overlap = vec![];
//     for i in a {
//         if !overlap.contains(&i) {
//             overlap.push(i);
//         }
//     }
//     for j in b {
//         if !overlap.contains(&j) {
//             overlap.push(j);
//         }
//     }
//     overlap
// }
