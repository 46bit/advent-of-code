use std::collections::HashSet;

#[aoc(day13, part1)]
fn part1(input: &str) -> i32 {
    let mut yx_dots: Vec<(usize, usize)> = vec![];
    let mut folds: Vec<(char, usize)> = vec![];
    for line in input.lines() {
        if line.len() > 11 && &line[0..11] == "fold along " {
            let space_split: Vec<_> = line.split(" ").collect();
            let equals_split: Vec<_> = space_split[2].split("=").collect();
            let axis = equals_split[0].chars().next().unwrap();
            let value = equals_split[1].parse::<usize>().unwrap();
            folds.push((axis, value));
        } else if line.len() > 0 {
            let comma_split: Vec<_> = line.split(",").collect();
            let x = comma_split[0].parse::<usize>().unwrap();
            let y = comma_split[1].parse::<usize>().unwrap();
            yx_dots.push((y, x));
        }
    }

    for (i, (axis, value)) in folds.into_iter().enumerate() {
        if axis == 'x' {
            for yx_dot in &mut yx_dots {
                assert_ne!(yx_dot.1, value);
                if yx_dot.1 > value {
                    yx_dot.1 -= 2 * (yx_dot.1 - value);
                }
            }
        } else if axis == 'y' {
            for yx_dot in &mut yx_dots {
                assert_ne!(yx_dot.0, value);
                if yx_dot.0 > value {
                    yx_dot.0 -= 2 * (yx_dot.0 - value);
                }
            }
        } else {
            unreachable!();
        }

        let mut unique_dots = HashSet::new();
        let mut y_max = 0;
        let mut x_max = 0;
        for yx_dot in &yx_dots {
            unique_dots.insert(yx_dot);
            if yx_dot.0 > y_max {
                y_max = yx_dot.0;
            }
            if yx_dot.1 > x_max {
                x_max = yx_dot.1;
            }
        }
        println!(
            "After fold #{}, {}={}: {}",
            i,
            axis,
            value,
            unique_dots.len()
        );
        for y in 0..=y_max {
            for x in 0..=x_max {
                if unique_dots.contains(&(y, x)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    return 0;
}

// FIXME: Recognise characters so can output correct answer?
// #[aoc(day13, part2)]
// fn part2(input: &str) -> i32 {
//     return 0;
// }
