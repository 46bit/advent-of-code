use either::Either;
use std::str::Lines;
use rayon::prelude::*;
use std::iter::Peekable;
use std::collections::{HashMap, VecDeque, HashSet};

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

pub struct Block {
    pub id: usize,
    pub block_div_z: i64,
    pub block_add_x: i64,
    pub block_add_y: i64,
}

impl Block {
    pub fn new(id: usize, lines: &mut Peekable<Lines>) -> Block {
        let mut block_lines = vec![];
        for _ in 0..18 {
            let line = lines.next().unwrap();
            //println!("  {}", line);
            block_lines.push(line);
        }

        let block_div_z = block_lines[4].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
        //println!("  block_div_z = {}", block_div_z);
        let block_add_x = block_lines[5].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
        //println!("  block_add_x = {}", block_add_x);
        let block_add_y = block_lines[15].split(" ").skip(2).next().unwrap().parse::<i64>().unwrap();
        //println!("  block_add_y = {}", block_add_y);

        Block { id, block_div_z, block_add_x, block_add_y }
    }

    pub fn evaluate(&self, input: i64, mut z: i64) -> i64 {
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

    pub fn ranges(&self, zs: HashSet<i64>) -> HashMap<i64, [i64; 9]> {
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

pub fn block_inputs_along_happy_path(blocks: &Vec<Block>) -> Vec<HashSet<((i64, u8), i64)>> {
    // Go forward through blocks and find out what input makes what output for each individual block.
    // This is efficient because we only check genuinely possible inputs to each block.
    let mut block_zs: Vec<_> = vec![];
    block_zs.push(blocks[0].ranges(vec![0].into_iter().collect()));
    println!("input->output relationships for block #0: {:?}", block_zs[0]);
    for i in 1..14 {
        println!("finding input->output relationships for block #{}", i);
        let zs = block_zs[i-1].values().cloned().flatten().collect();
        block_zs.push(blocks[i].ranges(zs));
    }
    println!("possible inputs to each block: {:?}", block_zs.iter().map(|l| l.len()).collect::<Vec<_>>());

    // Reverse through the blocks figuring out which inputs give z=0 from the last block.
    // This is efficient because we have a lookup table from all possible inputs to outputs.
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
                    good_inputs[i].insert(((*z_in, digit as u8), z_out));
                }
            }
        }
        println!("number of possible inputs that can lead to success for block #{} = {}", i, input_zs_that_can_succeed.len());
        output_zs_that_can_succeed = input_zs_that_can_succeed.clone();
    }
    return good_inputs;
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines().peekable();
    let mut blocks = vec![];
    for i in 0.. {
        println!("identifying block #{}", i);
        blocks.push(Block::new(i, &mut lines));
        if lines.peek() == None {
            break;
        }
    }
    assert_eq!(blocks.len(), 14);

    let block_inputs_along_happy_path = block_inputs_along_happy_path(&blocks);

    let ((_, first_digit), mut next_z_in) = block_inputs_along_happy_path[0].iter().max_by(|((_, digit1), _), ((_, digit2), _)| digit1.cmp(digit2)).unwrap();
    let mut max_digits = vec![first_digit];
    for i in 1..=13 {
        let ((_, new_digit), new_z_in) = block_inputs_along_happy_path[i].iter().filter(|((z_in, _), _)| {
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

    let mut max = 0;
    for i in 0..14 {
        max *= 10;
        max += *max_digits[i] as u64;
    }
    return max;
}

#[aoc(day24, part2)]
fn part2(input: &str) -> u64 {
    let mut lines = input.lines().peekable();
    let mut blocks = vec![];
    for i in 0.. {
        println!("identifying block #{}", i);
        blocks.push(Block::new(i, &mut lines));
        // FIXME: Stop requiring newlines between blocks in the input!!
        if lines.peek() == None {
            break;
        }
    }
    assert_eq!(blocks.len(), 14);

    let block_inputs_along_happy_path = block_inputs_along_happy_path(&blocks);

    let ((_, first_digit), mut next_z_in) = block_inputs_along_happy_path[0].iter().min_by(|((_, digit1), _), ((_, digit2), _)| digit1.cmp(digit2)).unwrap();
    let mut min_digits = vec![first_digit];
    for i in 1..=13 {
        let ((_, new_digit), new_z_in) = block_inputs_along_happy_path[i].iter().filter(|((z_in, _), _)| {
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

    let mut min = 0;
    for i in 0..14 {
        min *= 10;
        min += *min_digits[i] as u64;
    }
    return min;
}

#[cfg(test)]
mod tests {
    pub use super::*;

    const INPUT: &str = include_str!("../input/2021/day24.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 99394899891971);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 92171126131911);
    }

    #[test]
    fn test_alu_block_equivalency() {
        let inputs: VecDeque<i64> = (0..14).map(|i| (i * i) % 10).collect();

        let mut alu = ALU::default();
        simulate_on_alu(&mut alu, INPUT.clone(), inputs.clone());

        let mut lines = INPUT.lines().peekable();
        let mut blocks = vec![];
        for i in 0.. {
            println!("#{}", i);
            blocks.push(Block::new(i, &mut lines));
            if lines.peek() == None {
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