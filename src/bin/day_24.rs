use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use aoc2024::{input::get_all_numbers, sample};
use itertools::Itertools;
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

#[derive(Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn evaluate(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::And => a & b,
            Operation::Or => a | b,
            Operation::Xor => a ^ b,
        }
    }
}

#[derive(Clone, Copy)]
enum Input<'a> {
    Direct(char, usize),
    Intermediate(&'a str),
}

#[derive(Clone, Copy)]
struct Node<'a> {
    a: Input<'a>,
    b: Input<'a>,
    op: Operation,
}

#[derive(Clone)]
struct Device<'a> {
    x: u64,
    y: u64,
    input_bits: u64,
    nodes: HashMap<&'a str, Node<'a>>,
    output_keys: Vec<&'a str>,
}

impl<'a> Device<'a> {
    fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();

        let mut x = 0;
        let mut y = 0;
        let mut input_bits = 0;

        for (c, shift, value) in lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(Self::parse_initial)
        {
            input_bits = input_bits.max(shift);

            match c {
                'x' => x |= value << shift,
                'y' => y |= value << shift,
                x => panic!("Unexpected initial state: {}", x),
            }
        }

        let nodes: HashMap<&'a str, Node<'a>> = lines.map(Self::parse_node).collect();

        let mut output_keys: Vec<&'a str> = nodes
            .keys()
            .filter(|k| k.starts_with('z'))
            .copied()
            .collect();

        output_keys.sort();
        output_keys.reverse();

        Self {
            x,
            y,
            input_bits,
            nodes,
            output_keys,
        }
    }

    fn parse_initial(input: &'a str) -> (char, u64, u64) {
        let c = input.chars().next().unwrap();
        let nums = get_all_numbers::<u64>(input);

        (c, nums[0], nums[1])
    }

    fn parse_node(input: &'a str) -> (&'a str, Node<'a>) {
        let mut values = input.split_whitespace();
        let a = Self::parse_input(values.next().unwrap());
        let op_name = values.next().unwrap();
        let b = Self::parse_input(values.next().unwrap());

        let op = match op_name {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            x => panic!("Unsupported operation: {}", x),
        };

        // Don't need the arrow.
        values.next();

        let target = values.next().unwrap();

        (target, Node { a, b, op })
    }

    fn parse_input(input: &'a str) -> Input<'a> {
        let nums = get_all_numbers::<usize>(input);
        if input.starts_with('x') {
            Input::Direct('x', *nums.first().unwrap())
        } else if input.starts_with('y') {
            Input::Direct('y', *nums.first().unwrap())
        } else {
            Input::Intermediate(input)
        }
    }

    fn get_node_value(&self, input: Input<'a>) -> u64 {
        match input {
            Input::Direct('x', idx) => (self.x >> idx) & 1,
            Input::Direct('y', idx) => (self.y >> idx) & 1,
            Input::Intermediate(key) => {
                let node = self.nodes.get(key).unwrap();

                let a = self.get_node_value(node.a);
                let b = self.get_node_value(node.b);

                node.op.evaluate(a, b)
            }
            _ => panic!("Unexpected input!"),
        }
    }

    fn get_output(&self) -> u64 {
        let mut acc = 0;
        for k in self.output_keys.iter() {
            acc <<= 1;
            acc |= self.get_node_value(Input::Intermediate(k));
        }
        acc
    }

    fn swap(&mut self, n1: &'a str, n2: &'a str) {
        let temp = *self.nodes.get(n1).unwrap();
        self.nodes.insert(n1, *self.nodes.get(n2).unwrap());
        self.nodes.insert(n2, temp);
    }

    fn set_inputs(&mut self, x: u64, y: u64) {
        self.x = x;
        self.y = y;
    }
}

fn solve2(input: &str, swaps: usize, combine: fn(u64, u64) -> u64) -> String {
    let device = Device::parse(input);

    let max = (1 << (device.input_bits + 1)) - 1;

    let mut test_cases = vec![(0, 0), (max, max)];
    for i in 0..device.input_bits {
        let val = 1 << i;
        test_cases.push((val, val));
        test_cases.push((val, 0));
        test_cases.push((0, val));
    }

    let nodes: Vec<_> = device.nodes.keys().copied().collect();
    let pairs: Vec<(&str, &str)> = nodes.iter().copied().tuple_combinations().collect();

    let result: Vec<_> = pairs
        .iter()
        .combinations(swaps)
        .par_bridge()
        .filter(|swapset| {
            let mut cloned_device = device.clone();

            let mut seen = HashSet::new();
            for (n1, n2) in swapset {
                if seen.contains(n1) || seen.contains(n2) {
                    debug!("Skipping swapset due to duplicate node: {:?}", swapset);
                    return false;
                }
                cloned_device.swap(n1, n2);
                seen.insert(n1);
                seen.insert(n2);
            }

            let result = test_cases.iter().all(|(x, y)| {
                let expected = combine(*x, *y);
                cloned_device.set_inputs(*x, *y);

                let actual = cloned_device.get_output();
                expected == actual
            });

            if result {
                info!("Found potential solution. {:?}", swapset);
            }

            result
        })
        .collect();

    if result.len() != 1 {
        panic!("Bad result count. {:?}", result);
    }

    let mut nodes = Vec::new();
    let swaps = result.first().unwrap();
    for (n1, n2) in swaps {
        nodes.push(n1);
        nodes.push(n2);
    }

    nodes.sort();
    nodes.iter().join(",")
}

fn part1(input: &str) -> String {
    let device = Device::parse(input);
    let value = device.get_output();

    value.to_string()
}

fn part2(input: &str) -> String {
    solve2(input, 4, |a, b| a + b)
}

sample! {
    r"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
    part1 = "2024"
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_part2(input: &str) -> String {
        solve2(input, 2, |a, b| a & b)
    }

    sample! {
        r"
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00",
        test_part2 = "z00,z01,z02,z05"
    }
}
