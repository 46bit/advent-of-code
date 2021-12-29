use std::collections::HashSet;

const BORDER: isize = 500;

#[aoc(day20, part1)]
fn part1(input: &str) -> i64 {
    let mut lines = input.lines().peekable();

    let enhancement_string: Vec<char> = lines.next().unwrap().chars().collect();
    assert_eq!(enhancement_string.len(), 512);

    assert_eq!(lines.next().unwrap().len(), 0);

    let image_size = lines.peek().unwrap().len() as isize;
    let mut image = HashSet::new();
    for y in 0..image_size {
        let mut chars = lines.next().unwrap().chars();
        for x in 0..image_size {
            let char_ = chars.next().unwrap();
            let bit: u16 = match char_ {
                '#' => 1,
                '.' => 0,
                _ => panic!("wrong input char"),
            };
            // Default for uninitialised pixels is 0, so don't bother
            // storing any 0s.
            if bit == 1 {
                image.insert((y, x));
            }
        }
    }

    println!("pre");
    print(&image, image_size);
    println!();

    enhance(&mut image, image_size, &enhancement_string[..]);
    println!("post 1");
    print(&image, image_size);
    println!();

    enhance(&mut image, image_size, &enhancement_string[..]);
    println!("post 2");
    print(&image, image_size);
    println!();

    // Ignore the outer edge of the border because it has awkward
    // interactions with the uncomputed, all-zero space outside.
    let mut lit = 0;
    for y in -BORDER/2..(image_size + BORDER/2) {
        for x in -BORDER/2..(image_size + BORDER/2) {
            if image.contains(&(y, x)) {
                lit += 1;
            }
        }
    }
    return lit;
}

fn enhance(image: &mut HashSet<(isize, isize)>, image_size: isize, enhancement_string: &[char]) {
    let original_image = image.clone();
    for y in -BORDER..(image_size+BORDER) {
        for x in -BORDER..(image_size+BORDER) {
            let neighbours = vec![
                (y-1, x-1),
                (y-1, x),
                (y-1, x+1),
                (y,   x-1),
                (y,   x),
                (y,   x+1),
                (y+1, x-1),
                (y+1, x),
                (y+1, x+1),
            ];
            let mut bitstring: u16 = 0;
            for neighbour in neighbours {
                bitstring = (bitstring << 1) | if original_image.contains(&neighbour) {
                    1
                } else {
                    0
                };
            }
            if enhancement_string[bitstring as usize] == '#' {
                image.insert((y, x));
            } else {
                image.remove(&(y, x));
            }
        }
    }
}

fn print(image: &HashSet<(isize, isize)>, image_size: isize) {
    for y in -BORDER..(image_size+BORDER) {
        for x in -BORDER..(image_size+BORDER) {
            if image.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc(day20, part2)]
fn part2(input: &str) -> i64 {
    let mut lines = input.lines().peekable();

    let enhancement_string: Vec<char> = lines.next().unwrap().chars().collect();
    assert_eq!(enhancement_string.len(), 512);

    assert_eq!(lines.next().unwrap().len(), 0);

    let image_size = lines.peek().unwrap().len() as isize;
    let mut image = HashSet::new();
    for y in 0..image_size {
        let mut chars = lines.next().unwrap().chars();
        for x in 0..image_size {
            let char_ = chars.next().unwrap();
            let bit: u16 = match char_ {
                '#' => 1,
                '.' => 0,
                _ => panic!("wrong input char"),
            };
            // Default for uninitialised pixels is 0, so don't bother
            // storing any 0s.
            if bit == 1 {
                image.insert((y, x));
            }
        }
    }

    println!("pre");
    print(&image, image_size);
    println!();

    for i in 1..=50 {
        enhance(&mut image, image_size, &enhancement_string[..]);
        println!("post #{}", i);
        print(&image, image_size);
        println!();
    }

    // Ignore the outer edge of the border because it has awkward
    // interactions with the uncomputed, all-zero space outside.
    let mut lit = 0;
    for y in -BORDER/2..(image_size + BORDER/2) {
        for x in -BORDER/2..(image_size + BORDER/2) {
            if image.contains(&(y, x)) {
                lit += 1;
            }
        }
    }
    return lit;
}