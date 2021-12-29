use std::fmt;
use core::cmp::max;
use std::iter::Peekable;
use std::str::{FromStr, Chars};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Number {
    Pair {
        left: Box<Number>,
        right: Box<Number>,
    },
    Value(u64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Pair { left, right } => write!(f, "[{},{}]", left, right),
            Number::Value(value) => write!(f, "{}", value),
        }
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars().peekable();
        let n = Number::from_peekable_chars(&mut iter)?;
        if iter.next().is_some() {
            return Err("string was not fully parsed".to_string());
        }
        Ok(n)
    }
}

impl Number {
    fn from_peekable_chars(chars: &mut Peekable<Chars>) -> Result<Number, String> {
        let peeked_next_char = some_or_return!(chars.peek(), "string ended unexpectedly".to_string());
        if peeked_next_char.is_digit(10) {
            let mut digits = vec![];
            loop {
                match chars.peek() {
                    None => break,
                    Some(c) => {
                        if c.is_digit(10) {
                            digits.push(*c);
                            chars.next().unwrap();
                        } else if *c == ']' || *c == ',' {
                            break;
                        } else {
                            return Err(format!("unexpected character '{}' when a digit was expected", c));
                        }
                    }
                }
            }
            return Ok(ok_or_return_s!(digits.into_iter().collect::<String>().parse().map(Number::Value)));
        }

        if *peeked_next_char == '[' {
            chars.next().unwrap();
            let left = box ok_or_return!(Number::from_peekable_chars(chars));
            let separator = some_or_return!(chars.next(), "string ended unexpectedly when a comma was expected".to_string());
            if separator != ',' {
                return Err(format!("unexpected character '{}' when a comma was expected", separator));
            }
            let right = box ok_or_return!(Number::from_peekable_chars(chars));
            let peeked_terminator = some_or_return!(chars.peek(), "string ended unexpectedly when ] was expected".to_string());
            if *peeked_terminator != ']' {
                return Err(format!("unexpected character '{}' when ] was expected", peeked_terminator));
            }
            chars.next().unwrap();
            return Ok(Number::Pair {left, right});
        }

        return Err(format!("unexpected character '{}'", peeked_next_char));
    }

    pub fn is_pair(&self) -> bool {
        if let Number::Pair { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn is_value(&self) -> bool {
        if let Number::Value(_) = self {
            true
        } else {
            false
        }
    }

    pub fn unwrap_value(&self) -> u64 {
        if let Number::Value(n) = self {
            *n
        } else {
            panic!("panic: unwrap_value on a non-value");
        }
    }

    pub fn unwrap_left(&mut self) -> &mut Number {
        if let Number::Pair { left, .. } = self {
            left
        } else {
            panic!("panic: unwrap_left on a non-pair");
        }
    }

    pub fn unwrap_right(&mut self) -> &mut Number {
        if let Number::Pair { right, .. } = self {
            right
        } else {
            panic!("panic: unwrap_right on a non-pair");
        }
    }

    pub fn magnitude(&self) -> u64 {
        match self {
            Number::Value(n) => *n,
            Number::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    pub fn add(&mut self, other: Number) {
        self.add_without_reducing(other);
        while self.explode() || self.split() { }
    }

    fn add_without_reducing(&mut self, other: Number) {
        *self = Number::Pair {
            left: box self.clone(),
            right: box other,
        };
    }

    pub fn explode(&mut self) -> bool {
        // Can ignore remaining `to_left`/`to_right` return values that didn't
        // have anywhere to go
        self.explode_internal(0).exploded
    }

    // "To explode a pair, the pair's left value is added to the first regular 
    // number to the left of the exploding pair (if any), and the pair's right 
    // value is added to the first regular number to the right of the exploding 
    // pair (if any). Exploding pairs will always consist of two regular numbers. 
    // Then, the entire exploding pair is replaced with the regular number 0."
    fn explode_internal(&mut self, depth: u64) -> Exploded {
        if depth > 4 {
            unimplemented!();
        }
        if self.is_value() {
            return Exploded {
                exploded: false,
                to_left: None,
                to_right: None,
            };
        }

        if depth == 4 {
            let left = match self.unwrap_left() {
                Number::Value(value) => *value,
                _ => unimplemented!(),
            };
            let right = match self.unwrap_right() {
                Number::Value(value) => *value,
                _ => unimplemented!(),
            };

            *self = Number::Value(0);
            return Exploded {
                exploded: true,
                to_left: Some(left),
                to_right: Some(right),
            }
        }
        
        let mut explosion = Exploded::default();
        let Exploded { exploded, to_left, to_right } = self.unwrap_left().explode_internal(depth + 1);
        if exploded {
            self.unwrap_right().apply_exploded(&Exploded { exploded: true, to_left: to_right, to_right: None });
            explosion.exploded = exploded;
            explosion.to_left = to_left;
        } else {
            let Exploded { exploded, to_left, to_right } = self.unwrap_right().explode_internal(depth + 1);
            if exploded {
                self.unwrap_left().apply_exploded(&Exploded { exploded: true, to_left: None, to_right: to_left });
                explosion.exploded = exploded;
                explosion.to_right = to_right;
            }
        }
        return explosion;
    }

    fn apply_exploded(&mut self, exploded: &Exploded) {
        if exploded.to_left.is_some() && exploded.to_right.is_some() {
            panic!("cannot resolve both");
        }

        if let Some(to_left) = exploded.to_left {
            match self {
                Number::Value(value) => *value += to_left,
                Number::Pair { left, .. } => left.apply_exploded(exploded),
            }
        }
        if let Some(to_right) = exploded.to_right {
            match self {
                Number::Value(value) => *value += to_right,
                Number::Pair { right, .. } => right.apply_exploded(exploded),
            }
        }
    }

    // To split a regular number, replace it with a pair; the left element of 
    // the pair should be the regular number divided by two and rounded down, 
    // while the right element of the pair should be the regular number divided by 
    // two and rounded up. For example, 10 becomes [5,5], 11 becomes [5,6], 12 
    // becomes [6,6], and so on.
    pub fn split(&mut self) -> bool {
        match self {
            Number::Pair { left, right } => left.split() || right.split(),
            Number::Value(value) => {
                if *value >= 10 {
                    *self = Number::Pair {
                        left: box Number::Value(*value / 2),
                        right: box Number::Value((*value + 1) / 2),
                    };
                    return true;
                }
                return false;
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exploded {
    exploded: bool,
    to_left: Option<u64>,
    to_right: Option<u64>,
}

impl Default for Exploded {
    fn default() -> Exploded {
        Exploded {
            exploded: false,
            to_left: None,
            to_right: None,
        }
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u64 {
    return input.lines()
        .map(Number::from_str)
        .map(Result::unwrap)
        .reduce(|mut n, m| {
            n.add(m);
            n
        })
        .unwrap()
        .magnitude();
}

#[aoc(day18, part2)]
fn part2(input: &str) -> u64 {
    let numbers: Vec<_> = input.lines()
        .map(Number::from_str)
        .map(Result::unwrap)
        .collect();
    assert!(numbers.len() > 0);

    let mut largest_magnitude = u64::MIN;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let mut s = numbers[i].clone();
            s.add(numbers[j].clone());
            largest_magnitude = max(largest_magnitude, s.magnitude());
        }
    }
    return largest_magnitude;
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_explode_1() {
        // [[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4] (the 9 has no regular number to its left, so it is not added to any regular number).
        let mut number = Number::Pair {
            left: box Number::Pair {
                left: box Number::Pair {
                    left: box Number::Pair {
                        left: box Number::Pair {
                            left: box Number::Value(9),
                            right: box Number::Value(8),
                        },
                        right: box Number::Value(1),
                    },
                    right: box Number::Value(2),
                },
                right: box Number::Value(3),
            },
            right: box Number::Value(4),
        };
        assert_eq!(format!("{}", number), "[[[[[9,8],1],2],3],4]");

        assert!(number.explode());
        assert_eq!(format!("{}", number), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_explode_2() {
        // [7,[6,[5,[4,[3,2]]]]] becomes [7,[6,[5,[7,0]]]] (the 2 has no regular number to its right, and so it is not added to any regular number).
        let mut number = Number::Pair {
            left: box Number::Value(7),
            right: box Number::Pair {
                left: box Number::Value(6),
                right: box Number::Pair {
                    left: box Number::Value(5),
                    right: box Number::Pair {
                        left: box Number::Value(4),
                        right: box Number::Pair {
                            left: box Number::Value(3),
                            right: box Number::Value(2),
                        },
                    },
                },
            },
        };
        assert_eq!(format!("{}", number), "[7,[6,[5,[4,[3,2]]]]]");

        assert!(number.explode());
        assert_eq!(format!("{}", number), "[7,[6,[5,[7,0]]]]");
    }

    #[test]
    fn test_explode_3() {
        // [[6,[5,[4,[3,2]]]],1] becomes [[6,[5,[7,0]]],3].
        let mut number = Number::Pair {
            left: box Number::Pair {
                left: box Number::Value(6),
                right: box Number::Pair {
                    left: box Number::Value(5),
                    right: box Number::Pair {
                        left: box Number::Value(4),
                        right: box Number::Pair {
                            left: box Number::Value(3),
                            right: box Number::Value(2),
                        }
                    }
                }
            },
            right: box Number::Value(1),
        };
        assert_eq!(format!("{}", number), "[[6,[5,[4,[3,2]]]],1]");

        assert!(number.explode());
        assert_eq!(format!("{}", number), "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn test_explode_4() {
        // [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] (the pair [3,2] is unaffected because the pair [7,3] is further to the left; [3,2] would explode on the next action).

        let mut number = Number::Pair {
            left: box Number::Pair {
                left: box Number::Value(3),
                right: box Number::Pair {
                    left: box Number::Value(2),
                    right: box Number::Pair {
                        left: box Number::Value(1),
                        right: box Number::Pair {
                            left: box Number::Value(7),
                            right: box Number::Value(3),
                        },
                    },
                },
            },
            right: box Number::Pair {
                left: box Number::Value(6),
                right: box Number::Pair {
                    left: box Number::Value(5),
                    right: box Number::Pair {
                        left: box Number::Value(4),
                        right: box Number::Pair {
                            left: box Number::Value(3),
                            right: box Number::Value(2),
                        },
                    },
                },
            },
        };
        assert_eq!(format!("{}", number), "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");

        assert!(number.explode());
        assert_eq!(format!("{}", number), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    }

    #[test]
    fn test_explode_5() {
        // [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[7,0]]]].

        let mut number = Number::Pair {
            left: box Number::Pair {
                left: box Number::Value(3),
                right: box Number::Pair {
                    left: box Number::Value(2),
                    right: box Number::Pair {
                        left: box Number::Value(8),
                        right: box Number::Value(0),
                    },
                },
            },
            right: box Number::Pair {
                left: box Number::Value(9),
                right: box Number::Pair {
                    left: box Number::Value(5),
                    right: box Number::Pair {
                        left: box Number::Value(4),
                        right: box Number::Pair {
                            left: box Number::Value(3),
                            right: box Number::Value(2),
                        },
                    },
                },
            },
        };
        assert_eq!(format!("{}", number), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        assert!(number.explode());
        assert_eq!(format!("{}", number), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_split() {
        let mut n = Number::from_str("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
        assert_eq!(format!("{}", n), "[[[[0,7],4],[15,[0,13]]],[1,1]]");

        assert!(n.split());
        assert_eq!(format!("{}", n), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");

        assert!(n.split());
        assert_eq!(format!("{}", n), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

        assert!(!n.split());
    }

    #[test]
    fn test_add() {
        // Here is the process of finding the reduced result of 
        // [[[[4,3],4],4],[7,[[8,4],9]]] + [1,1]:
        //
        // after addition: [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
        // after explode:  [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
        // after explode:  [[[[0,7],4],[15,[0,13]]],[1,1]]
        // after split:    [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
        // after split:    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
        // after explode:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
        //
        // Once no reduce actions apply, the snailfish number that remains is the actual result of 
        // the addition operation: [[[[0,7],4],[[7,8],[6,0]]],[8,1]].
        let mut n = Number::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        assert_eq!(format!("{}", n), "[[[[4,3],4],4],[7,[[8,4],9]]]");
        let m = Number::from_str("[1,1]").unwrap();
        assert_eq!(format!("{}", m), "[1,1]");
        n.add(m);
        assert_eq!(format!("{}", n), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_magnitude() {
        // the magnitude of [9,1] is 3*9 + 2*1 = 29
        // the magnitude of [1,9] is 3*1 + 2*9 = 21
        // [[1,2],[[3,4],5]] becomes 143
        // [[[[0,7],4],[[7,8],[6,0]]],[8,1]] becomes 1384
        // [[[[1,1],[2,2]],[3,3]],[4,4]] becomes 445
        // [[[[3,0],[5,3]],[4,4]],[5,5]] becomes 791
        // [[[[5,0],[7,4]],[5,5]],[6,6]] becomes 1137
        // [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] becomes 3488
        let expectations = [
            ("[9,1]", 29),
            ("[1,9]", 21),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ];
        for (number_str, expected_magnitude) in expectations {
            let n = Number::from_str(number_str).unwrap();
            assert_eq!(format!("{}", n), number_str);
            assert_eq!(n.magnitude(), expected_magnitude);
        }
    }

    #[test]
    fn test_worked_example() {
        let mut input = "
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]

[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]

[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]

[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
[7,[5,[[3,8],[1,4]]]]
[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]

[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
[[2,[2,2]],[8,[8,1]]]
[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]

[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
[2,9]
[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]

[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]

[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
[[[5,[7,4]],7],1]
[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]

[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
[[[[4,2],2],6],[8,7]]
[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
".lines();

        input.next().unwrap();
        for i in 0..9 {
            let mut a = input.next().map(Number::from_str).unwrap().unwrap();
            let b = input.next().map(Number::from_str).unwrap().unwrap();
            let expected_result = input.next().map(Number::from_str).unwrap().unwrap();
            println!("#{}: {} + {}", i, a, b);
            a.add(b);
            assert_eq!(format!("{}", a), format!("{}", expected_result));

            input.next();
        }
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"), 4140);
    }
}