use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Board {
    numbers: [[u8; 5]; 5],
    drawn: [[bool; 5]; 5],
}

impl Board {
    fn new() -> Board {
        Board {
            numbers: [[0; 5]; 5],
            drawn: [[false; 5]; 5],
        }
    }

    fn update_with_drawn_number(&mut self, drawn: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == drawn {
                    self.drawn[i][j] = true;
                }
            }
        }
    }

    fn won(&self) -> bool {
        for i in 0..5 {
            if self.drawn[i] == [true; 5] {
                return true;
            }
        }

        let mut column_true;
        for j in 0..5 {
            column_true = true;
            for i in 0..5 {
                if !self.drawn[i][j] {
                    column_true = false;
                }
            }
            if column_true {
                return true;
            }
        }

        return false;
    }

    fn sum_of_undrawn(&self) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.drawn[i][j] {
                    sum += self.numbers[i][j] as u32;
                }
            }
        }
        return sum;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut j = 0;
        for l in self.numbers {
            let mut i = 0;
            for b in l {
                ok_or_return!(write!(f, "{:2}", b));
                if self.drawn[j][i] {
                    ok_or_return!(write!(f, "*"));
                } else if i < 4 {
                    ok_or_return!(write!(f, " "));
                }
                i += 1;
            }
            if j < 4 {
                ok_or_return!(write!(f, "\n"));
            }
            j += 1;
        }
        Ok(())
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        let mut l = 0;
        for line in s.lines() {
            let tokens = line.split(" ").filter(|t| t.len() > 0);
            for (i, token) in tokens.enumerate() {
                if i > 4 {
                    return Err("too many non-empty tokens on line".to_string());
                }
                let number = match token.parse::<u8>() {
                    Ok(n) => n,
                    Err(e) => return Err(format!("{}", e)),
                };
                board.numbers[l][i] = number;
            }
            l += 1;
        }
        Ok(board)
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let drawn = lines
        .next()
        .unwrap()
        .split(",")
        .map(|t| t.parse::<u8>())
        .map(Result::unwrap);
    assert_eq!(lines.next().unwrap(), "");

    // let board_lines = lines.take(5).collect::<Vec<_>>().join("\n");
    // let mut board = Board::from_str(board_lines.as_str()).unwrap();
    // println!("{}", board);
    // assert_eq!(format!("{}", board), board_lines);
    // assert!(!board.won());
    // board.drawn[0] = [true; 5];
    // assert!(board.won());
    // board.drawn[0] = [false; 5];
    // assert!(!board.won());
    // for i in 0..5 {
    //     board.drawn[i][3] = true;
    // }
    // assert!(board.won());

    let mut boards = vec![];
    loop {
        let mut board_lines = vec![];
        for _ in 0..5 {
            board_lines.push(lines.next().unwrap());
        }
        let board_lines = board_lines.join("\n");
        let board = Board::from_str(board_lines.as_str()).unwrap();
        println!("{}\n", board);
        boards.push(board);
        match lines.next() {
            Some(line) => {
                assert_eq!(line, "");
            }
            None => break,
        }
    }

    let mut won = None;
    let mut last_drawn = None;
    for draw in drawn {
        last_drawn = Some(draw);
        for board in &mut boards {
            board.update_with_drawn_number(draw);
            if board.won() {
                println!("-----\nWON\n{}\nWON\n-----", board);
                won = Some(*board);
                break;
            }
        }
        if won.is_some() {
            break;
        }
    }

    let winning_board = won.unwrap();
    return (winning_board.sum_of_undrawn() as i32) * (last_drawn.unwrap() as i32);
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let mut lines = input.lines();
    let drawn = lines
        .next()
        .unwrap()
        .split(",")
        .map(|t| t.parse::<u8>())
        .map(Result::unwrap);
    assert_eq!(lines.next().unwrap(), "");

    let mut boards = vec![];
    loop {
        let mut board_lines = vec![];
        for _ in 0..5 {
            board_lines.push(lines.next().unwrap());
        }
        let board_lines = board_lines.join("\n");
        let board = Board::from_str(board_lines.as_str()).unwrap();
        println!("{}\n", board);
        boards.push(board);
        match lines.next() {
            Some(line) => {
                assert_eq!(line, "");
            }
            None => break,
        }
    }

    let mut unwon = boards.len();
    let mut last_drawn = None;
    let mut latest_won = None;
    for draw in drawn {
        last_drawn = Some(draw);

        for board in &mut boards {
            if board.won() {
                continue;
            }
            board.update_with_drawn_number(draw);
            if board.won() {
                unwon -= 1;
                latest_won = Some(*board);
                if unwon == 0 {
                    break;
                }
            }
        }

        if unwon == 0 {
            break;
        }
    }

    let winning_board = latest_won.unwrap();
    return (winning_board.sum_of_undrawn() as i32) * (last_drawn.unwrap() as i32);
}
