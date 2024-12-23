use std::collections::{BTreeSet, HashMap, HashSet};

use aoc2024::sample;
use itertools::Itertools;
use tracing::debug;

fn main() {
    aoc2024::run(part1, Some(part2));
}

type NetworkGroup<'a> = BTreeSet<BTreeSet<&'a str>>;

fn to_triple_group(groups: &mut NetworkGroup) {
    let mut long_groups: Vec<_> = groups.iter().filter(|g| g.len() > 3).cloned().collect();

    while let Some(group) = long_groups.pop() {
        groups.remove(&group);

        let triples = group
            .into_iter()
            .combinations(3)
            .map(BTreeSet::<&str>::from_iter);
        groups.extend(triples);
    }
}

struct Network<'a> {
    connections: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Network<'a> {
    fn new() -> Self {
        let connections = HashMap::new();

        Self { connections }
    }

    fn add_connection(&mut self, input: &'a str) {
        let (a, b) = input.split_at(2);

        self.connections.entry(a).or_default().insert(&b[1..]);
        self.connections.entry(&b[1..]).or_default().insert(a);
    }

    fn is_connected(&self, a: &'a str, b: &'a str) -> bool {
        self.connections.get(a).unwrap().contains(b)
    }

    fn find_groups(&self) -> NetworkGroup<'a> {
        let mut groups = NetworkGroup::new();

        for (computer, children) in self.connections.iter() {
            debug!("Finding groups starting at {}", computer);
            let mut local_groups: Vec<_> = children
                .iter()
                .map(|c| BTreeSet::<&'a str>::from_iter(vec![*computer, *c]))
                .collect();

            for group in local_groups.iter_mut() {
                for c in children.iter() {
                    if !group.contains(c) && group.iter().all(|gc| self.is_connected(gc, c)) {
                        group.insert(c);
                    }
                }
            }

            groups.extend(local_groups);
        }

        groups
    }
}

fn part1(input: &str) -> String {
    let mut network = Network::new();
    for connection in input.lines() {
        network.add_connection(connection);
    }

    let mut groups = network.find_groups();

    debug!("Original groups.");
    for g in groups.iter() {
        debug!("Group: {:?}", g);
    }
    to_triple_group(&mut groups);
    debug!("Triple groups.");
    for g in groups.iter() {
        debug!("Group: {:?}", g);
    }

    let target_groups = groups
        .iter()
        .filter(|g| g.iter().any(|s| s.starts_with('t') && g.len() == 3))
        .count();
    target_groups.to_string()
}

fn part2(input: &str) -> String {
    let mut network = Network::new();
    for connection in input.lines() {
        network.add_connection(connection);
    }

    let groups = network.find_groups();
    let longest = groups
        .iter()
        .max_by_key(|g| g.len())
        .map(|g| {
            let mut nodes = Vec::from_iter(g);
            nodes.sort();
            nodes
        })
        .unwrap();

    let password = longest.into_iter().join(",");
    password.to_string()
}

sample! {
    r"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
    part1 = "7",
    part2 = "co,de,ka,ta"
}
