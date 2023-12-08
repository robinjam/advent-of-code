use std::collections::HashMap;

use num_integer::lcm;

pub fn day08(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> String {
    Network::from(input)
        .navigate("AAA", |node| node == "ZZZ")
        .to_string()
}

fn part2(input: &str) -> String {
    let network = Network::from(input);
    let start_nodes = network.nodes.keys().filter(|node| node.ends_with("A"));
    let cycle_lengths =
        start_nodes.map(|start| network.navigate(start, |node| node.ends_with("Z")));
    cycle_lengths.fold(1, lcm).to_string()
}

struct Network<'input> {
    instructions: &'input str,
    nodes: HashMap<&'input str, (&'input str, &'input str)>,
}

impl Network<'_> {
    fn navigate<F>(&self, start_node: &str, is_end_node: F) -> usize
    where
        F: Fn(&str) -> bool,
    {
        let mut current_node = start_node;
        for (i, instruction) in self.instructions.chars().cycle().enumerate() {
            if is_end_node(current_node) {
                return i;
            }
            let (lhs, rhs) = self.nodes[&current_node];
            match instruction {
                'L' => current_node = lhs,
                'R' => current_node = rhs,
                _ => panic!(),
            }
        }

        unreachable!()
    }
}

impl<'input> From<&'input str> for Network<'input> {
    fn from(s: &'input str) -> Network<'input> {
        let mut lines = s.lines();
        let instructions = lines.next().unwrap();
        let nodes = lines
            .skip(1)
            .map(|line| {
                let name = &line[0..3];
                let lhs = &line[7..10];
                let rhs = &line[12..15];
                (name, (lhs, rhs))
            })
            .collect();
        Network {
            instructions,
            nodes,
        }
    }
}

#[test]
fn test() {
    assert_eq!(part1(include_str!("../examples/08_1.txt")), "2");
    assert_eq!(part1(include_str!("../examples/08_2.txt")), "6");
    assert_eq!(part2(include_str!("../examples/08_3.txt")), "6");
}
