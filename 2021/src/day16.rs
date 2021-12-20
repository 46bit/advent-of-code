#[derive(Clone, Debug)]
struct BitIterator {
    bytes: Vec<u8>,
    bit: usize,
    bits_consumed: usize,
}

impl BitIterator {
    fn new(bytes: Vec<u8>) -> BitIterator {
        BitIterator {
            bytes,
            bit: 7,
            bits_consumed: 0,
        }
    }
}

impl Iterator for BitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.len() == 0 {
            return None;
        }

        let current = self.bytes[0];
        let bit = (current >> self.bit) & 1;
        if self.bit == 0 {
            // FIXME: O(n), stop doing this!!
            self.bytes.remove(0);
            self.bit = 7;
        } else {
            self.bit -= 1;
        }
        self.bits_consumed += 1;
        Some(bit)
    }
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u64 {
    let mut bytes = vec![];
    let mut start = true;
    for char_ in input.chars() {
        let four_bits: u8 = match char_ {
            '0' => 0b0000,
            '1' => 0b0001,
            '2' => 0b0010,
            '3' => 0b0011,
            '4' => 0b0100,
            '5' => 0b0101,
            '6' => 0b0110,
            '7' => 0b0111,
            '8' => 0b1000,
            '9' => 0b1001,
            'A' => 0b1010,
            'B' => 0b1011,
            'C' => 0b1100,
            'D' => 0b1101,
            'E' => 0b1110,
            'F' => 0b1111,
            _ => unreachable!(),
        };
        if start {
            bytes.push(four_bits << 4);
            start = false;
        } else {
            let l = bytes.len();
            bytes[l - 1] |= four_bits;
            start = true;
        }
    }
    println!("{:?}", bytes);
    assert!(start);

    let mut bit_iterator = BitIterator::new(bytes);
    let packet = parse_packets(&mut bit_iterator);
    println!("{:?}", packet);

    return packet.sum_version_numbers();
}

#[derive(Clone, Debug)]
enum Packet {
    Number(u8, u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    fn sum_version_numbers(&self) -> u64 {
        match self {
            &Packet::Number(version_number, _, _) => version_number as u64,
            &Packet::Operator(version_number, _, ref subpackets) => {
                version_number as u64
                    + subpackets
                        .iter()
                        .map(Packet::sum_version_numbers)
                        .sum::<u64>()
            }
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            &Packet::Number(_, _, n) => n as u64,
            &Packet::Operator(_, type_id, ref subpackets) => match type_id {
                0 => subpackets.iter().map(Packet::evaluate).sum::<u64>(),
                1 => subpackets.iter().map(Packet::evaluate).product::<u64>(),
                2 => subpackets.iter().map(Packet::evaluate).min().unwrap() as u64,
                3 => subpackets.iter().map(Packet::evaluate).max().unwrap() as u64,
                5 => {
                    assert_eq!(subpackets.len(), 2);
                    let first = subpackets[0].evaluate() as u64;
                    let second = subpackets[1].evaluate() as u64;
                    if first > second {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                6 => {
                    assert_eq!(subpackets.len(), 2);
                    let first = subpackets[0].evaluate() as u64;
                    let second = subpackets[1].evaluate() as u64;
                    if first < second {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                7 => {
                    assert_eq!(subpackets.len(), 2);
                    let first = subpackets[0].evaluate() as u64;
                    let second = subpackets[1].evaluate() as u64;
                    if first == second {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                _ => panic!("unexpected type id"),
            },
        }
    }
}

fn parse_packets(bit_iterator: &mut BitIterator) -> Packet {
    let version = (bit_iterator.next().unwrap() << 2)
        | (bit_iterator.next().unwrap() << 1)
        | bit_iterator.next().unwrap();
    let type_id = (bit_iterator.next().unwrap() << 2)
        | (bit_iterator.next().unwrap() << 1)
        | bit_iterator.next().unwrap();
    println!("version = {}", version);
    println!("type_id = {}", type_id);
    if type_id == 4 {
        let mut final_number_packet = false;
        let mut number: u64 = 0;
        while !final_number_packet {
            final_number_packet = bit_iterator.next().unwrap() == 0;
            number = (number << 4)
                | ((bit_iterator.next().unwrap() << 3)
                    | (bit_iterator.next().unwrap() << 2)
                    | (bit_iterator.next().unwrap() << 1)
                    | bit_iterator.next().unwrap()) as u64;
        }
        println!("number = {}", number);
        return Packet::Number(version, type_id, number);
    } else {
        let mut subpackets = vec![];

        let operator_type_id = bit_iterator.next().unwrap();
        if operator_type_id == 0 {
            let mut subpacket_length_in_bits: usize = 0;
            for _ in 0..15 {
                subpacket_length_in_bits =
                    (subpacket_length_in_bits << 1) | bit_iterator.next().unwrap() as usize;
            }
            println!("subpacket_length_in_bits = {}", subpacket_length_in_bits);

            let start = bit_iterator.bits_consumed;
            loop {
                if bit_iterator.bits_consumed - start == subpacket_length_in_bits {
                    break;
                }
                if bit_iterator.bits_consumed - start > subpacket_length_in_bits {
                    panic!("read too far");
                }
                let subpacket = parse_packets(bit_iterator);
                subpackets.push(subpacket);
            }
        } else {
            let mut number_of_subpackets: usize = 0;
            for _ in 0..11 {
                number_of_subpackets =
                    (number_of_subpackets << 1) | bit_iterator.next().unwrap() as usize;
            }
            println!("number_of_subpackets = {}", number_of_subpackets);
            for _ in 0..number_of_subpackets {
                let subpacket = parse_packets(bit_iterator);
                subpackets.push(subpacket);
            }
        }

        return Packet::Operator(version, type_id, subpackets);
    }
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u64 {
    let mut bytes = vec![];
    let mut start = true;
    for char_ in input.chars() {
        let four_bits: u8 = match char_ {
            '0' => 0b0000,
            '1' => 0b0001,
            '2' => 0b0010,
            '3' => 0b0011,
            '4' => 0b0100,
            '5' => 0b0101,
            '6' => 0b0110,
            '7' => 0b0111,
            '8' => 0b1000,
            '9' => 0b1001,
            'A' => 0b1010,
            'B' => 0b1011,
            'C' => 0b1100,
            'D' => 0b1101,
            'E' => 0b1110,
            'F' => 0b1111,
            _ => unreachable!(),
        };
        if start {
            bytes.push(four_bits << 4);
            start = false;
        } else {
            let l = bytes.len();
            bytes[l - 1] |= four_bits;
            start = true;
        }
    }
    println!("{:?}", bytes);
    assert!(start);

    let mut bit_iterator = BitIterator::new(bytes);
    let packet = parse_packets(&mut bit_iterator);
    println!("{:?}", packet);

    return packet.evaluate();
}
