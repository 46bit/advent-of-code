use either::Either;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ALU {
    registers: HashMap<char, i64>,
}

impl Default for ALU {
    fn default() -> ALU {
        let mut registers = HashMap::new();
        registers.insert('w', 0);
        registers.insert('x', 0);
        registers.insert('y', 0);
        registers.insert('z', 0);
        ALU { registers }
    }
}

impl ALU {
    // Does not support INP instructions
    pub fn exec(&mut self, name: &str, register: char, register_or_value: Either<char, i64>) {
        match name {
            "add" => self.exec_add(register, register_or_value),
            "mul" => self.exec_mul(register, register_or_value),
            "div" => self.exec_div(register, register_or_value),
            "mod" => self.exec_mod(register, register_or_value),
            "eql" => self.exec_eql(register, register_or_value),
            _ => panic!("unknown instruction type"),
        }
    }

    pub fn exec_inp(&mut self, register: char, value: i64) {
        self.registers.insert(register, value);
    }

    pub fn exec_add(&mut self, register: char, register_or_value: Either<char, i64>) {
        let result = self.registers[&register] + match register_or_value {
            Either::Left(c) => self.registers[&c],
            Either::Right(v) => v,
        };
        self.registers.insert(register, result);
    }

    pub fn exec_mul(&mut self, register: char, register_or_value: Either<char, i64>) {
        let result = self.registers[&register] * match register_or_value {
            Either::Left(c) => self.registers[&c],
            Either::Right(v) => v,
        };
        self.registers.insert(register, result);
    }

    pub fn exec_div(&mut self, register: char, register_or_value: Either<char, i64>) {
        let result = self.registers[&register] / match register_or_value {
            Either::Left(c) => self.registers[&c],
            Either::Right(v) => v,
        };
        self.registers.insert(register, result);
    }

    pub fn exec_mod(&mut self, register: char, register_or_value: Either<char, i64>) {
        let result = self.registers[&register] % match register_or_value {
            Either::Left(c) => self.registers[&c],
            Either::Right(v) => v,
        };
        self.registers.insert(register, result);
    }

    pub fn exec_eql(&mut self, register: char, register_or_value: Either<char, i64>) {
        let rhs = match register_or_value {
            Either::Left(c) => self.registers[&c],
            Either::Right(v) => v,
        };
        if self.registers[&register] == rhs {
            self.registers.insert(register, 1);
        } else {
            self.registers.insert(register, 0);
        }
    }
}

pub fn simulate_on_alu(alu: &mut ALU, instructions: &str, mut inputs: VecDeque<i64>) {
    for line in instructions.lines() {
        if line.len() == 0 {
            continue;
        }
        let space_separated: Vec<_> = line.split(" ").collect();
        let name = space_separated[0];
        assert_eq!(name.len(), 3);
        let register = space_separated[1].chars().nth(0).unwrap();
        assert_eq!(space_separated[1].len(), 1);
        if name == "inp" {
            println!("{} {}", name, register);
            alu.exec_inp(register, inputs.pop_front().unwrap());
        } else {
            let register_or_value_str = space_separated[2];
            let register_or_value = match register_or_value_str.parse::<i64>() {
                Ok(v) => Either::Right(v),
                Err(_) => {
                    assert_eq!(register_or_value_str.len(), 1);
                    Either::Left(register_or_value_str.chars().nth(0).unwrap())
                },
            };
            println!("{} {} {}", name, register, register_or_value);
            alu.exec(name, register, register_or_value);
        }
    }
    println!("{:?}", alu.registers);
    if alu.registers[&'z'] == 1 {
        println!("VALID");
    } else {
        println!("INVALID");
    }
}

pub struct Block<'a> {
    id: usize,
    lines: Vec<&'a str>,
    block_div_z: i64,
    block_add_x: i64,
    block_add_y: i64,
}

impl<'a> Block<'a> {
    fn evaluate(&self, input: i64, mut z: i64) -> i64 {
        // Manually decompiled:
        // if (z % 26) + BLOCK_ADD_X == BLOCK_INPUT {
        //     z = z / BLOCK_DIV_Z
        // } else {
        //     z = z / BLOCK_DIV_Z
        //     z = z * 26 + BLOCK_INPUT + BLOCK_ADD_Y
        // }
        if (z % 26) + self.block_add_x == input {
            z /= self.block_div_z;
        } else {
            z /= self.block_div_z;
            z *= 26;
            z += input;
            z += self.block_add_y
        }
        return z;
    }

    fn ranges(&self, zs: HashSet<i64>) -> HashMap<i64, [i64; 9]> {
        //let mut zr = vec![];
        zs.into_par_iter()
            .map(|z| {
                let mut z_outs = [0; 9];
                for digit in 1..=9 {
                    let mut z2: i64 = z / self.block_div_z;
                    if (z % 26) + self.block_add_x != digit as i64 {
                        z2 = z2 * 26 + digit as i64 + self.block_add_y;
                    }
                    z_outs[digit - 1] = z2;
                }
                (z, z_outs)
            })
            .collect()
    }
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut blocks = vec![];
    for i in 0.. {
        println!("#{}", i);
        let mut block_lines = vec![];
        for _ in 0..18 {
            let line = lines.next().unwrap();
            //println!("  {}", line);
            block_lines.push(line);
        }

        let block_div_z = block_lines[4].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
        println!("  block_div_z = {}", block_div_z);
        let block_add_x = block_lines[5].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
        println!("  block_add_x = {}", block_add_x);
        let block_add_y = block_lines[15].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
        println!("  block_add_y = {}", block_add_y);

        blocks.push(Block {
            block_div_z,
            block_add_x,
            block_add_y,
            id: i,
            lines: block_lines,
        });

        if lines.next() == None {
            break;
        }
    }
    assert_eq!(blocks.len(), 14);

    //for (j, block) in blocks.iter().enumerate() {
    //}

    let mut block_zs: Vec<_> = vec![];
    block_zs.push(blocks[0].ranges(vec![0].into_iter().collect()));
    println!("{:?}", block_zs[0]);
    for i in 1..14 {
        println!("{}", i);
        let zs = block_zs[i-1].values().cloned().flatten().collect();
        block_zs.push(blocks[i].ranges(zs));
    }
    println!("{:?}", block_zs.iter().map(|l| l.len()).collect::<Vec<_>>());

    // let mut ways_into_final_block_that_succeed = HashSet::new();
    // for (z_in, z_outs) in &block_zs[13] {
    //     for digit in 1..=9 {
    //         if z_outs[digit-1] == 0 {
    //             ways_into_final_block_that_succeed.insert((z_in, digit));
    //         }
    //     }
    // }
    // println!("ways_into_final_block_that_succeed = {:?}", ways_into_final_block_that_succeed);
    // println!("ways_into_final_block_that_succeed.len() = {}", ways_into_final_block_that_succeed.len());
    // let final_block_input_xs_that_can_succeed: HashSet<_> = ways_into_final_block_that_succeed.iter().map(|(z_in, _)| *z_in).collect();

    // let mut second_to_last_block_input_xs_that_can_succeed = HashSet::new();
    // for (z_in, z_outs) in &block_zs[12] {
    //     for digit in 1..=9 {
    //         if final_block_input_xs_that_can_succeed.contains(&z_outs[digit - 1]) {
    //             second_to_last_block_input_xs_that_can_succeed.insert(z_in);
    //         }
    //     }
    // }
    // println!("second_to_last_block_input_xs_that_can_succeed = {:?}", second_to_last_block_input_xs_that_can_succeed);
    // println!("second_to_last_block_input_xs_that_can_succeed.len() = {}", second_to_last_block_input_xs_that_can_succeed.len());

    // Reverse through the blocks figuring out which Z values can lead to a successful outcome.
    // This step discards which digits work.
    let mut output_zs_that_can_succeed = HashSet::new();
    output_zs_that_can_succeed.insert(0);
    let mut good_inputs = vec![HashSet::new(); 14];
    for j in 0..=13 {
        let i = 13 - j;
        let mut input_zs_that_can_succeed = HashSet::new();
        for (z_in, z_outs) in &block_zs[i] {
            for digit in 1..=9 {
                let z_out = z_outs[digit - 1];
                if output_zs_that_can_succeed.contains(&z_out) {
                    input_zs_that_can_succeed.insert(*z_in);
                    good_inputs[i].insert(((*z_in, digit), z_out));
                }
            }
        }
        println!("#{}: input_zs_that_can_succeed.len() = {}", i, input_zs_that_can_succeed.len());
        output_zs_that_can_succeed = input_zs_that_can_succeed.clone();
    }

    let ((_, first_digit), mut next_z_in) = good_inputs[0].iter().next().unwrap();
    let mut digits = vec![first_digit];
    for i in 1..=13 {
        let ((_, new_digit), new_z_in) = good_inputs[i].iter().filter(|((z_in, digit), z_out)| {
            *z_in == next_z_in
        }).next().unwrap();
        next_z_in = *new_z_in;
        digits.push(new_digit);
    }
    println!("digits = {:?}", digits);
    let mut z = 0;
    for i in 0..14 {
        z = blocks[i].evaluate(*digits[i] as i64, z);
    }
    assert_eq!(z, 0);

    let ((_, first_digit), mut next_z_in) = good_inputs[0].iter().max_by(|((_, digit1), _), ((_, digit2), _)| digit1.cmp(digit2)).unwrap();
    let mut max_digits = vec![first_digit];
    for i in 1..=13 {
        let ((_, new_digit), new_z_in) = good_inputs[i].iter().filter(|((z_in, digit), z_out)| {
            *z_in == next_z_in
        }).max_by(|((_, digit1), _), ((_, digit2), _)| digit1.cmp(digit2)).unwrap();
        next_z_in = *new_z_in;
        max_digits.push(new_digit);
    }
    println!("max_digits = {:?}", max_digits);
    let mut z = 0;
    for i in 0..14 {
        z = blocks[i].evaluate(*max_digits[i] as i64, z);
    }
    assert_eq!(z, 0);

    let ((_, first_digit), mut next_z_in) = good_inputs[0].iter().min_by(|((_, digit1), _), ((_, digit2), _)| digit1.cmp(digit2)).unwrap();
    let mut min_digits = vec![first_digit];
    for i in 1..=13 {
        let ((_, new_digit), new_z_in) = good_inputs[i].iter().filter(|((z_in, digit), z_out)| {
            *z_in == next_z_in
        }).min_by(|((_, digit1), _), ((_, digit2), _)| digit1.cmp(digit2)).unwrap();
        next_z_in = *new_z_in;
        min_digits.push(new_digit);
    }
    println!("min_digits = {:?}", min_digits);
    let mut z = 0;
    for i in 0..14 {
        z = blocks[i].evaluate(*min_digits[i] as i64, z);
    }
    assert_eq!(z, 0);

    return 0;

    //let mut cache: Arc<RwLock<HashMap<(i64, usize, i64), i64>>> = Arc::new(RwLock::new(HashMap::new()));

    const TOTAL: u64 = 99999999999999 + 1;
    const BATCH_SIZE: u64 = 1000000;
    const ITER_ROUNDS: u64 = TOTAL / BATCH_SIZE;

    const CACHE_SIZE: usize = 999999999;
    let mut cache = box [[[0; CACHE_SIZE]; 10]; 14];
    //let mut cache = HashMap::new();
    for (j, block) in blocks.iter().enumerate() {
        println!("{}", j);
        for z in 0..CACHE_SIZE {
            for digit in 1..=9 {
                //let block_inputs = (j, (digit, z));
                cache[j][digit][z] = block.evaluate(digit as i64, z as i64);
            }
        }
    }

    let j = AtomicU64::new(0);
    let skipped = AtomicU64::new(0);
    (0..ITER_ROUNDS).into_par_iter().for_each(|_| {
        let i0 = j.fetch_add(BATCH_SIZE, Ordering::Relaxed);
        let progress = 100.0 * (i0 as f64) / (TOTAL as f64);
        let skippish = 100.0 * (skipped.load(Ordering::Relaxed) as f64) / (i0 as f64);
        //let cache_size = rcache.len();
        println!("progress: {}th iteration ({}% of search space) ({}% skipped)", i0, progress, skippish);

        //let rcache = cache.clone(); //cache.read().unwrap().clone();

        'iter: for z in 0..BATCH_SIZE {
            let i: u64 = i0 + z;

            let mut input = 99999999999999 - i;
            let input2 = input;
            let mut digits = [0; 14];
            let mut n = 13;
            while input > 0 && n >= 0 {
                digits[n] = input % 10;
                if digits[n] == 0 {
                    //skipped += 1;
                    skipped.fetch_add(1, Ordering::Relaxed);
                    continue 'iter;
                }
                input /= 10;
                n -= 1;
            }

            let mut z: i64 = 0;
            for (j, block) in blocks.iter().enumerate() {
                if z > 99999999999999 {
                    skipped.fetch_add(1, Ordering::Relaxed);
                    continue 'iter;
                }
                let digit = digits[j] as i64;
                //z = rcache[&(j, (digit, z))];
                z = cache[j][digit as usize][z as usize];
            }
            if z == 0 {
                println!("SOLUTION: {}", input2);
            }
        }
    });

    return 0;
}

// #[aoc(day24, part2)]
// fn part2(input: &str) -> i32 {
//     return 0;
// }

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_alu_block_equivalency() {
        let instructions = "inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 26
add x -1
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y

inp w
mul x 0
add x z
mod x 26
div z 26
add x -14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y";
        let inputs: VecDeque<i64> = (0..14).map(|i| (i * i) % 10).collect();

        let mut alu = ALU::default();
        simulate_on_alu(&mut alu, instructions.clone(), inputs.clone());

        let mut lines = instructions.lines();
        let mut blocks = vec![];
        for i in 0.. {
            println!("#{}", i);
            let mut block_lines: Vec<_> = (0..18).map(|_| {
                lines.next().unwrap()
            }).collect();

            let block_div_z = block_lines[4].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
            println!("  block_div_z = {}", block_div_z);
            let block_add_x = block_lines[5].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
            println!("  block_add_x = {}", block_add_x);
            let block_add_y = block_lines[15].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
            println!("  block_add_y = {}", block_add_y);

            blocks.push(Block {
                block_div_z,
                block_add_x,
                block_add_y,
                id: i,
                lines: block_lines,
            });

            if lines.next() == None {
                break;
            }
        }
        assert_eq!(blocks.len(), 14);
        let mut z = 0;
        for i in 0..14 {
            z = blocks[i].evaluate(inputs[i], z);
        }

        assert_eq!(alu.registers[&'z'], z);
    }
}