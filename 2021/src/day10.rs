#[aoc(day10, part1)]
fn part1(input: &str) -> i32 {
    let mut valid_complete_lines = vec![];
    let mut invalid_score = 0;
    for line in input.lines() {
        let mut stack = vec![];
        let mut valid = true;
        for char_ in line.chars() {
            match char_ {
                '{' => stack.push('{'),
                '(' => stack.push('('),
                '[' => stack.push('['),
                '<' => stack.push('<'),
                '}' => {
                    if stack[stack.len() - 1] != '{' {
                        valid = false;
                        invalid_score += 1197;
                        break;
                    }
                    stack.pop();
                }
                ')' => {
                    if stack[stack.len() - 1] != '(' {
                        valid = false;
                        invalid_score += 3;
                        break;
                    }
                    stack.pop();
                }
                ']' => {
                    if stack[stack.len() - 1] != '[' {
                        valid = false;
                        invalid_score += 57;
                        break;
                    }
                    stack.pop();
                }
                '>' => {
                    if stack[stack.len() - 1] != '<' {
                        valid = false;
                        invalid_score += 25137;
                        break;
                    }
                    stack.pop();
                }
                _ => valid = false,
            }
        }
        let incomplete = stack.len() > 0;
        if valid && !incomplete {
            valid_complete_lines.push(line);
        }
    }
    println!("{:?}", valid_complete_lines);
    return invalid_score;
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u64 {
    let mut valid_lines = vec![];
    for line in input.lines() {
        let mut stack = vec![];
        let mut valid = true;
        for char_ in line.chars() {
            match char_ {
                '{' => stack.push('{'),
                '(' => stack.push('('),
                '[' => stack.push('['),
                '<' => stack.push('<'),
                '}' => {
                    if stack[stack.len() - 1] != '{' {
                        valid = false;
                        break;
                    }
                    stack.pop();
                }
                ')' => {
                    if stack[stack.len() - 1] != '(' {
                        valid = false;
                        break;
                    }
                    stack.pop();
                }
                ']' => {
                    if stack[stack.len() - 1] != '[' {
                        valid = false;
                        break;
                    }
                    stack.pop();
                }
                '>' => {
                    if stack[stack.len() - 1] != '<' {
                        valid = false;
                        break;
                    }
                    stack.pop();
                }
                _ => valid = false,
            }
        }
        let incomplete = stack.len() > 0;
        assert!(incomplete || !valid);
        if valid {
            valid_lines.push((line, stack));
        }
    }
    println!("{:?}", valid_lines);

    let mut completion_scores = vec![];
    for (_, mut stack) in valid_lines {
        let mut completion_score: u64 = 0;
        while let Some(c) = stack.pop() {
            //valid_line.append(c);
            completion_score *= 5;
            completion_score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };
        }
        completion_scores.push(completion_score);
    }
    completion_scores.sort();
    return completion_scores[completion_scores.len() / 2];
}
