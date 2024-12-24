use std::collections::HashMap;

use aoc2024::sample;
use tracing::info;

fn main() {
    aoc2024::run(part1, None);
}

#[derive(Clone, Copy)]
struct Node<'a> {
    a: &'a str,
    b: &'a str,
    op: &'a str,
}

struct Device<'a> {
    cache: HashMap<&'a str, u64>,
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Device<'a> {
    fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();

        let init = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(Self::parse_initial)
            .collect();

        let nodes = lines.map(Self::parse_node).collect();

        Self { cache: init, nodes }
    }

    fn parse_initial(input: &'a str) -> (&'a str, u64) {
        let mut values = input.split(": ");
        let name = values.next().unwrap();
        let value = values.next().unwrap().parse::<u64>().unwrap();

        (name, value)
    }

    fn parse_node(input: &'a str) -> (&'a str, Node<'a>) {
        let mut values = input.split_whitespace();
        let a = values.next().unwrap();
        let op = values.next().unwrap();
        let b = values.next().unwrap();

        // Don't need the arrow.
        values.next();

        let target = values.next().unwrap();

        (target, Node { a, b, op })
    }

    fn get_state(&mut self, name: &'a str) -> u64 {
        if let Some(cached) = self.cache.get(name) {
            return *cached;
        }

        let node = *self.nodes.get(name).unwrap();

        let aval = self.get_state(node.a);
        let bval = self.get_state(node.b);

        let result = match node.op {
            "AND" => aval & bval,
            "OR" => aval | bval,
            "XOR" => aval ^ bval,
            x => panic!("Unrecognized operation: {}", x),
        };

        self.cache.insert(name, result);
        result
    }

    fn get_value(&mut self, symbol: char) -> u64 {
        let mut bits = Vec::new();

        self.cache
            .keys()
            .filter(|k| k.starts_with(symbol))
            .for_each(|k| bits.push(*k));

        self.nodes
            .keys()
            .filter(|k| k.starts_with(symbol))
            .for_each(|k| bits.push(*k));

        bits.sort();
        bits.dedup();
        bits.reverse();
        info!("Bits are {:?}", bits);

        let mut acc = 0;
        for bit in bits {
            acc <<= 1;
            acc |= self.get_state(bit);
        }

        acc
    }
}

fn part1(input: &str) -> String {
    let mut device = Device::parse(input);
    let value = device.get_value('z');

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
