use std::collections::HashMap;

use aoc2024::{input::get_all_numbers, sample};

fn main() {
    aoc2024::run(part1, None);
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

struct Device<'a> {
    x: u64,
    y: u64,
    nodes: HashMap<&'a str, Node<'a>>,
    output_keys: Vec<&'a str>,
}

impl<'a> Device<'a> {
    fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();

        let mut x = 0;
        let mut y = 0;

        for (c, shift, value) in lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(Self::parse_initial)
        {
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
}

fn part1(input: &str) -> String {
    let device = Device::parse(input);
    let value = device.get_output();

    value.to_string()
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
