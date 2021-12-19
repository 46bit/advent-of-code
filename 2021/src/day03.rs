use std::fmt;
use std::str::FromStr;

// Using a normal integer type would definitely be faster, but
// playing with const generics is cool.
#[derive(Copy, Clone, Debug)]
struct Bits<const W: usize>([bool; W]);

impl<const W: usize> Bits<W> {
    fn new() -> Bits<W> {
        Bits([false; W])
    }

    fn u32(&self) -> u32 {
        let mut n: u32 = 0;
        for i in 0..BITWIDTH {
            if self.0[i] {
                n |= 1 << (BITWIDTH - 1 - i);
            }
        }
        n
    }
}

impl<const W: usize> fmt::Display for Bits<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .into_iter()
                .map(|b| match b {
                    true => '1',
                    false => '0',
                })
                .collect::<String>()
        )
    }
}

impl<const W: usize> FromStr for Bits<W> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = Bits([false; W]);
        if s.len() != BITWIDTH {
            return Err(format!(
                "input string of wrong length: was {} but required {}",
                s.len(),
                BITWIDTH
            ));
        }
        for (i, c) in s.chars().enumerate() {
            bits.0[i] = match c {
                '0' => false,
                '1' => true,
                e => return Err(format!("unknown character: {}", e)),
            };
        }
        Ok(bits)
    }
}

const BITWIDTH: usize = 12;

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let mut lines = input.lines().peekable();
    let actual_bitwidth = lines.peek().unwrap().len();
    assert_eq!(actual_bitwidth, BITWIDTH);
    let bits: Vec<Bits<BITWIDTH>> = lines
        .map(|l| Bits::from_str(l))
        .map(Result::unwrap)
        .collect();
    // let recreated_input = bits
    //     .iter()
    //     .map(|m| format!("{}", m))
    //     .collect::<Vec<_>>()
    //     .join("\n");
    // assert_eq!(input, recreated_input);

    let mut gamma: Bits<BITWIDTH> = Bits::new();
    let mut epsilon: Bits<BITWIDTH> = Bits::new();
    for i in 0..BITWIDTH {
        let mut n = 0;
        for b in &bits {
            if b.0[i] {
                n += 1;
            }
        }
        // FIXME: Problem description is ambiguous about when 50% are true/false
        epsilon.0[i] = true;
        if n > bits.len() / 2 {
            gamma.0[i] = true;
            epsilon.0[i] = false;
        }
    }
    // let b: Bits<BITWIDTH> = Bits::from_str("000000010111").unwrap();
    // assert_eq!(b.i64(), 23);

    return (gamma.u32() as i32) * (epsilon.u32() as i32);
}

// FIXME: Gives wrong answer
#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let mut lines = input.lines().peekable();
    let actual_bitwidth = lines.peek().unwrap().len();
    assert_eq!(actual_bitwidth, BITWIDTH);
    let bits: Vec<Bits<BITWIDTH>> = lines
        .map(|l| Bits::from_str(l))
        .map(Result::unwrap)
        .collect();

    let mut gamma: Bits<BITWIDTH> = Bits::new();
    let mut epsilon: Bits<BITWIDTH> = Bits::new();
    for i in 0..BITWIDTH {
        let mut n = 0;
        for b in &bits {
            if b.0[i] {
                n += 1;
            }
        }
        // FIXME: Problem description is ambiguous about when 50% are true/false
        epsilon.0[i] = true;
        if n >= (bits.len() + 1) / 2 {
            gamma.0[i] = true;
            epsilon.0[i] = false;
        }
    }

    let mut oxygen_generator_words: Vec<_> = bits.clone();
    let mut i = 0;
    let mut remaining;
    while oxygen_generator_words.len() > 1 && i < BITWIDTH {
        remaining = oxygen_generator_words.len();
        oxygen_generator_words = oxygen_generator_words
            .into_iter()
            .filter(|word| {
                if remaining > 1 && word.0[i] != gamma.0[i] {
                    remaining -= 1;
                    return false;
                }
                return true;
            })
            .collect();
        i += 1;
    }
    assert_eq!(oxygen_generator_words.len(), 1);

    let mut co2_generator_words: Vec<_> = bits.clone();
    i = 0;
    while co2_generator_words.len() > 1 && i < BITWIDTH {
        remaining = co2_generator_words.len();
        co2_generator_words = co2_generator_words
            .into_iter()
            .filter(|word| {
                if remaining > 1 && word.0[i] != epsilon.0[i] {
                    remaining -= 1;
                    return false;
                }
                return true;
            })
            .collect();
        i += 1;
    }
    assert_eq!(co2_generator_words.len(), 1);

    return (oxygen_generator_words[0].u32() as i32) * (co2_generator_words[0].u32() as i32);
}
