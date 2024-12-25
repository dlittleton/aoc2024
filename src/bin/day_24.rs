use std::collections::{HashMap, HashSet};

use aoc2024::{input::get_all_numbers, sample};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    fn get_node_value(&self, input: Input<'a>, depth: usize) -> Option<u64> {
        // Arbitrary cutoff to avoid a loop.
        if depth > 100 {
            return None;
        }

        match input {
            Input::Direct('x', idx) => Some((self.x >> idx) & 1),
            Input::Direct('y', idx) => Some((self.y >> idx) & 1),
            Input::Intermediate(key) => {
                let node = self.nodes.get(key).unwrap();

                let a = self.get_node_value(node.a, depth + 1);
                let b = self.get_node_value(node.b, depth + 1);

                match (a, b) {
                    (Some(a), Some(b)) => Some(node.op.evaluate(a, b)),
                    _ => None,
                }
            }
            _ => panic!("Unexpected input!"),
        }
    }

    fn get_output(&self) -> Option<u64> {
        let mut acc = 0;
        for k in self.output_keys.iter() {
            acc <<= 1;

            if let Some(val) = self.get_node_value(Input::Intermediate(k), 0) {
                acc |= val;
            } else {
                return None;
            }
        }

        Some(acc)
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

    fn get_children(&self, bit: u64) -> HashSet<&'a str> {
        let name = format!("z{:02}", bit);
        let mut to_visit = vec![name.as_str()];

        let mut result = HashSet::new();

        while let Some(n) = to_visit.pop() {
            let (k, v) = self.nodes.get_key_value(n).unwrap();
            result.insert(*k);

            if let Input::Intermediate(key) = v.a {
                to_visit.push(key);
            }

            if let Input::Intermediate(key) = v.b {
                to_visit.push(key);
            }
        }

        result
    }
}

struct Solver<'a> {
    _swaps: usize, // Ended up not needing to know the swap count ahead of time
    device: Device<'a>,
    swappable: HashSet<&'a str>,
    swapped_nodes: Vec<&'a str>,
}

impl<'a> Solver<'a> {
    fn new(swaps: usize, device: Device<'a>) -> Self {
        let swappable = device.nodes.keys().copied().collect();
        let swapped_nodes = Vec::new();

        Self {
            _swaps: swaps,
            device,
            swappable,
            swapped_nodes,
        }
    }

    fn check_bit(bit: u64, device: &mut Device<'a>) -> bool {
        let prev = 1 << (bit - 1);
        let value = 1 << bit;
        let both = prev + value;
        let cases = [
            (value, 0),
            (value, value),
            (prev, prev),
            (prev, both),
            (both, both),
        ];

        for (x, y) in cases {
            device.set_inputs(x, y);
            let target = x + y;
            if let Some(actual) = device.get_output() {
                if target != actual {
                    debug!("Problem detected at bit {}", bit);
                    debug!("\t{} + {} != {}. Actual value: {}", x, y, target, actual);
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn mark_children_safe(&mut self, bit: u64) {
        for child in self.device.get_children(bit) {
            self.swappable.remove(child);
        }
    }

    fn find_swap(&mut self, bit: u64) {
        let mut children = self.device.get_children(bit);
        children.retain(|t| self.swappable.contains(t));

        info!("Potentially swappable children: {:?}", children);

        let temp_swappable = self.swappable.clone();

        for c in children {
            for other in temp_swappable.iter() {
                if c == *other {
                    continue;
                }

                let mut cloned_device = self.device.clone();
                cloned_device.swap(c, other);

                if Self::check_bit(bit, &mut cloned_device) {
                    info!("Swapping {} and {} works!", c, other);

                    self.swapped_nodes.push(c);
                    self.swapped_nodes.push(other);
                    self.swappable.remove(c);
                    self.swappable.remove(other);
                    self.device = cloned_device;
                    return;
                }
            }
        }

        info!("Done finding swap?");
    }

    fn run(&mut self) {
        for bit in 1..self.device.input_bits {
            if Self::check_bit(bit, &mut self.device.clone()) {
                self.mark_children_safe(bit);
            } else {
                info!("Stopping at {}", bit);
                self.find_swap(bit);
            }
        }
    }
}

fn solve2(input: &str, swaps: usize) -> String {
    let device = Device::parse(input);
    let mut solver = Solver::new(swaps, device);
    solver.run();

    solver.swapped_nodes.sort();
    solver.swapped_nodes.join(",")
}

fn part1(input: &str) -> String {
    let device = Device::parse(input);
    let value = device.get_output();

    value.unwrap().to_string()
}

fn part2(input: &str) -> String {
    solve2(input, 4)
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
