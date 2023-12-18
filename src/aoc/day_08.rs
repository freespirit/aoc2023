use std::collections::HashMap;

struct Node {
    name: String,
    left: String,
    right: String,
}

fn build_nodes_map(map: &str) -> HashMap<String, Node> {
    // build a list of nodes from the input map
    let nodes: Vec<Node> = map.lines()
        .skip(1)
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let parts = line.split('=').collect::<Vec<&str>>();
            let name = parts[0].trim();
            let left_right = parts[1].trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .splitn(2, ", ")
                .collect::<Vec<&str>>();
            let (left, right) = (left_right[0], left_right[1]);
            Node {
                name: name.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            }
        })
        .collect();

    // build the map
    let mut result = HashMap::new();
    for node in nodes {
        result.insert(node.name.clone(), node);
    }

    result
}

pub fn solve_part_1(input: &str) -> i64 {
    // let map: HashMap<String, Node> = HashMap::new();
    let directions = input.lines().next().unwrap().chars();
    // map a node name to a node
    let map: HashMap<String, Node> = build_nodes_map(input);

    let mut steps = 0;
    let mut next_node = map.get("AAA").unwrap();
    let mut dir_iter = directions.into_iter().cycle();
    while &next_node.name != "ZZZ" {
        let direction = dir_iter.next().unwrap();
        let next_name = match direction {
            'L' => &next_node.left,
            _ => &next_node.right,
        };
        next_node = map.get(next_name).unwrap();
        steps += 1;
    }

    steps
}

pub fn solve_part_2(input: &str) -> i64 {
    // let map: HashMap<String, Node> = HashMap::new();
    let directions: Vec<char> = input.lines().next().unwrap().chars().collect();
    // map a node name to a node
    let map: HashMap<String, Node> = build_nodes_map(input);

    let start_nodes: Vec<&Node> = map.values()
        .filter(|&node| node.name.ends_with('A'))
        .collect();
    let mut steps: Vec<i64> = vec![0; start_nodes.len()];

    for (pos, &node) in start_nodes.iter().enumerate() {
        let mut dir_iter = directions.iter().cycle();
        let mut next_node = node;
        while !next_node.name.ends_with('Z') {
            let direction = dir_iter.next().unwrap();
            let next_name = match direction {
                'L' => &next_node.left,
                _ => &next_node.right,
            };
            next_node = map.get(next_name).unwrap();
            steps[pos] += 1;
        }
    }

    steps.iter().fold(1, |a, &b| lcm(a, b))
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let map = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve_part_1(map), 6);
    }

    #[test]
    fn part_2() {
        let map = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(solve_part_2(map), 6);
    }
}