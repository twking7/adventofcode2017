use regex::Regex;
use std::collections::{HashMap, HashSet};

type Line = (String, usize, Vec<String>);

#[derive(Clone, Debug)]
struct Node {
    name: String,
    weight: usize,
    children: Vec<Node>,
}

impl Node {
    fn from_line(line: &Line) -> Self {
        Node { name: line.0.to_string(), weight: line.1, children: Vec::new() }
    }
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"\A(\w+) \((\d+)\)").unwrap();
    let lines: Vec<Line> = input.split('\n').map(|l| parse_line(l, &re)).collect();
    let mut all_names = HashSet::new();
    let mut child_names = HashSet::new();

    for line in lines {
        all_names.insert(line.0);
        for child in line.2 {
            child_names.insert(child);
        }
    }

    let diff: Vec<_> = all_names.difference(&child_names).collect();
    let names: Vec<String> = diff.iter().map(|n| n.to_string()).collect();
    names.first().unwrap().to_string()
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"\A(\w+) \((\d+)\)").unwrap();
    let lines: Vec<Line> = input.split('\n').map(|l| parse_line(l, &re)).collect();
    let mut line_map = HashMap::new();
    let mut all_names = HashSet::new();
    let mut child_names = HashSet::new();

    for line in lines {
        all_names.insert(line.0.clone());
        for child_name in line.2.clone() {
            child_names.insert(child_name);
        }
        line_map.insert(line.0.clone(), line);
    }

    let diff: Vec<_> = all_names.difference(&child_names).collect();
    let names: Vec<String> = diff.iter().map(|n| n.to_string()).collect();
    let root_name = names.first().unwrap();
    let root_line = line_map.get(root_name).unwrap();

    let root_node = build_tree(&root_line, &line_map);
    match find_imbalanced(&root_node) {
        Some(n) => calc_new_weight(&n),
        None => 0
    }
}

fn build_tree(root_line: &Line, line_map: &HashMap<String, Line>) -> Node {
    let mut node = Node::from_line(root_line);

    for child_name in root_line.2.to_owned() {
        let child_line = line_map.get(&child_name).unwrap();
        let child_node = build_tree(child_line, line_map);
        node.children.push(child_node);
    }

    node
}

fn calc_new_weight(node: &Node) -> usize {
    let mut children: Vec<(usize, &Node)> = node.children.iter().map(|c| (total_weight(c), c)).collect();
    children.sort_by(|a, b| a.0.cmp(&b.0));
    let heaviest = children.last().unwrap();
    let lightest = children.first().unwrap();
    let diff = heaviest.0 - lightest.0;

    heaviest.1.weight - diff
}

fn find_imbalanced(root: &Node) -> Option<&Node> {
    if root.children.len() == 0 {
        return None
    }

    if !is_balanced(root) && root.children.iter().all(|c| is_balanced(c)) {
        return Some(&root)
    } else {
        for child in root.children.iter() {
            if let Some(n) = find_imbalanced(&child) {
                return Some(n)
            }
        }
        None
    }
}

fn is_balanced(root: &Node) -> bool {
    if root.children.len() <= 1 {
        true
    } else {
        let mut child_weights: Vec<usize> = root.children.iter().map(|n| total_weight(n)).collect();
        child_weights.sort_by(|a, b| a.cmp(b));
        child_weights.first().unwrap() == child_weights.last().unwrap()
    }
}

fn total_weight(node: &Node) -> usize {
    node.weight + node.children.iter().map(|n| total_weight(n)).sum::<usize>()
}

fn parse_line(line: &str, regex: &Regex) -> Line {
    let pieces: Vec<&str> = line.split("->").collect();
    let left = pieces[0];

    let caps = regex.captures(left).unwrap();
    let name = caps[1].to_string();
    let weight = caps[2].parse().unwrap();

    let children: Vec<String> = match pieces.get(1) {
        Some(piece) => piece.trim().split(",").map(|s| s.trim().to_string()).collect(),
        None => Vec::new(),
    };

    (name, weight, children)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        assert_eq!(part1(input), "tknk".to_string());
    }

    #[test]
    fn test_part2() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!(part2(input), 60);
    }
}
