#![feature(slice_rotate)]

use std::io::{stdin, Read};

fn main() {
  let mut input = String::new();
  stdin().read_to_string(&mut input).unwrap();
  let output = d2a(&input);
  println!("{:?}", output);
}

fn d2a(input: &str) -> u32 {
  matrix(input).into_iter().map(|mut row| {
    row.sort_unstable();
    match (row.first(), row.last()) {
      (Some(least), Some(most)) => most - least,
      _ => 0,
    }
  }).sum()
}

fn matrix(digit_string: &str) -> Vec<Vec<u32>> {
  digit_string.lines().map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect()).collect()
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn provided_testcase() {
        assert_eq!(d2a("5 1 9 5\n 7 5 3\n 2 4 6 8"), 18);
    }

    #[test]
    fn multidigit_testcase() {
        assert_eq!(d2a("59 1 9 5\n 7 5 3\n 2 4 6 8"), 68);
    }
}
