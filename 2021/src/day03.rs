//use std::fmt;
//use std::str::FromStr;

// // Using a normal integer type would definitely be faster, but
// // playing with const generics is cool.
// #[derive(Copy, Clone, Debug)]
// struct Bits<const W: usize>([bool; W]);

// impl<const W: usize> Bits<W> {
//     fn new() -> Bits<W> {
//         Bits([false; W])
//     }

//     fn u32(&self) -> u32 {
//         let mut n: u32 = 0;
//         for i in 0..BITWIDTH {
//             if self.0[i] {
//                 n |= 1 << (BITWIDTH - 1 - i);
//             }
//         }
//         n
//     }
// }

// impl<const W: usize> fmt::Display for Bits<W> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             self.0
//                 .into_iter()
//                 .map(|b| match b {
//                     true => '1',
//                     false => '0',
//                 })
//                 .collect::<String>()
//         )
//     }
// }

// impl<const W: usize> FromStr for Bits<W> {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut bits = Bits([false; W]);
//         if s.len() != BITWIDTH {
//             return Err(format!(
//                 "input string of wrong length: was {} but required {}",
//                 s.len(),
//                 BITWIDTH
//             ));
//         }
//         for (i, c) in s.chars().enumerate() {
//             bits.0[i] = match c {
//                 '0' => false,
//                 '1' => true,
//                 e => return Err(format!("unknown character: {}", e)),
//             };
//         }
//         Ok(bits)
//     }
// }

const BITWIDTH: usize = 12;

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let mut lines = input.lines().peekable();
    let actual_bitwidth = lines.peek().unwrap().len();
    assert_eq!(actual_bitwidth, BITWIDTH);

    let numbers: Vec<u16> = lines.map(|l| u16::from_str_radix(l, 2).unwrap()).collect();
    
    let o2 = o2_rating(numbers.clone());
    let co2 = co2_rating(numbers.clone());
    println!("o2_rating = {}", o2);
    println!("co2_rating = {}", co2);
    return (o2 as u64) * (co2 as u64);
}

fn o2_rating(mut numbers: Vec<u16>) -> u16 {
    for b in 0..BITWIDTH {
        let shift = BITWIDTH - 1 - b;
        let mut ones = 0;
        for number in &numbers {
            let bit = (number >> shift) & 1;
            if bit == 1 {
                ones += 1;
            }
        }

        let mut expect = 0;
        if ones * 2 >= numbers.len() {
            expect = 1;
        }
        numbers = numbers.into_iter().filter(|n| (n >> shift) & 1 == expect).collect();
        match numbers.len() {
            0 => panic!("ran out of numbers"),
            1 => break,
            _ => {},
        }
    }
    assert_eq!(numbers.len(), 1);
    return numbers[0];
}

fn co2_rating(mut numbers: Vec<u16>) -> u16 {
    for b in 0..BITWIDTH {
        let shift = BITWIDTH - 1 - b;
        let mut zeroes = 0;
        let mut ones = 0;
        for number in &numbers {
            let bit = (number >> shift) & 1;
            if bit == 0 {
                zeroes += 1;
            } else {
                ones += 1;
            }
        }

        // FIXME: Sort out this logic. And merge with o2_rating
        let expect;
        if zeroes > ones {
            expect = 1;
        } else if ones > zeroes {
            expect = 0;
        } else {
            expect = 0;
        }
        numbers = numbers.into_iter().filter(|n| (n >> shift) & 1 == expect).collect();
        match numbers.len() {
            0 => panic!("ran out of numbers"),
            1 => break,
            _ => {},
        }
    }
    assert_eq!(numbers.len(), 1);
    return numbers[0];
}
