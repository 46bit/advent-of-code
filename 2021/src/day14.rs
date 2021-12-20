use std::collections::HashMap;

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut polymer = lines.next().unwrap().to_owned();
    assert_eq!(lines.next().unwrap().len(), 0);
    let mut insertions = HashMap::new();
    for line in lines {
        let arrow_split: Vec<_> = line.split(" -> ").collect();
        let pair = arrow_split[0].to_owned();
        let insert = arrow_split[1].chars().next().unwrap();
        assert!(!insertions.contains_key(&pair));
        insertions.insert(pair, insert);
    }
    println!("polymer = {}", polymer);
    println!("insertions = {:?}", insertions);

    for i in 0..10 {
        let mut chars = polymer.chars().peekable();
        let mut new_polymer = vec![];
        loop {
            let first = chars.next().unwrap();
            let second = match chars.peek() {
                Some(c) => *c,
                None => {
                    new_polymer.push(first);
                    break;
                }
            };
            let insert = insertions[&format!("{}{}", first, second)];
            new_polymer.append(&mut vec![first, insert]);
        }
        polymer = new_polymer.into_iter().collect();

        println!("After #{}: {}", i, polymer);
    }

    let mut commonality: HashMap<char, u64> = HashMap::new();
    for c in polymer.chars() {
        *commonality.entry(c).or_insert(0) += 1;
    }
    println!("{:?}", commonality);

    let mut sorted_commonalities: Vec<_> = commonality.into_iter().collect();
    sorted_commonalities.sort_by(|(_, a), (_, b)| a.cmp(b));
    println!("{:?}", sorted_commonalities);
    return sorted_commonalities[sorted_commonalities.len() - 1].1 - sorted_commonalities[0].1;
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap().to_owned();
    assert_eq!(lines.next().unwrap().len(), 0);
    let mut insertions = HashMap::new();
    for line in lines {
        let arrow_split: Vec<_> = line.split(" -> ").collect();
        let pair = arrow_split[0].to_owned();
        let insert = arrow_split[1].chars().next().unwrap();
        assert!(!insertions.contains_key(&pair));
        insertions.insert(pair, insert);
    }
    println!("polymer = {}", polymer);
    println!("insertions = {:?}", insertions);

    let mut pairs: HashMap<String, u64> = HashMap::new();
    let mut chars = polymer.chars().peekable();
    loop {
        let first = chars.next().unwrap();
        let second = match chars.peek() {
            Some(c) => *c,
            None => break,
        };
        *pairs.entry(format!("{}{}", first, second)).or_insert(0) += 1;
    }
    println!("pairs = {:?}", pairs);

    for i in 1..=40 {
        let previous_pairs: Vec<_> = pairs.drain().collect();
        for (pair, n) in previous_pairs {
            let insertion = insertions[&pair];
            let mut pair_chars = pair.chars();
            let first = pair_chars.next().unwrap();
            let second = pair_chars.next().unwrap();
            let first_new_pair = format!("{}{}", first, insertion);
            let second_new_pair = format!("{}{}", insertion, second);
            *pairs.entry(first_new_pair).or_insert(0) += n;
            *pairs.entry(second_new_pair).or_insert(0) += n;
        }

        println!("#{}: pairs = {:?}", i, pairs);
    }

    let mut commonality: HashMap<char, u64> = HashMap::new();
    for (pair, n) in pairs {
        let mut pair_chars = pair.chars();
        let first = pair_chars.next().unwrap();
        let second = pair_chars.next().unwrap();
        *commonality.entry(first).or_insert(0) += n;
        *commonality.entry(second).or_insert(0) += n;
    }
    // The above code double counts all characters except the first and last in
    // the polymer. We manually correct this by duplicating the first and last below.
    let polymer_vec: Vec<char> = polymer.chars().collect();
    *commonality.entry(polymer_vec[0]).or_insert(0) += 1;
    *commonality
        .entry(polymer_vec[polymer_vec.len() - 1])
        .or_insert(0) += 1;
    for (_, v) in &mut commonality {
        *v /= 2;
    }
    println!("{:?}", commonality);

    let mut sorted_commonalities: Vec<_> = commonality.into_iter().collect();
    sorted_commonalities.sort_by(|(_, a), (_, b)| a.cmp(b));
    println!("{:?}", sorted_commonalities);
    return sorted_commonalities[sorted_commonalities.len() - 1].1 - sorted_commonalities[0].1;
}
