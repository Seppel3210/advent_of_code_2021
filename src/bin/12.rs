use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn main() {
    let input = advent_of_code_2021::input("12");
    let mut nodes = HashMap::new();
    for (a, b) in input.lines().map(parse_line) {
        nodes.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
        nodes.entry(b).or_insert(Vec::new()).push(a);
    }

    let paths = visit_adjacent(&nodes, HashSet::new(), Cave::Start, Vec::new());
    println!("{paths}");
}

fn visit_adjacent(
    nodes: &HashMap<Cave, Vec<Cave>>,
    mut small_visited: HashSet<Cave>,
    node: Cave,
    mut path: Vec<Cave>,
) -> usize {
    let mut paths = 0;
    path.push(node.clone());
    if matches!(node, Cave::Small(_)) {
        small_visited.insert(node.clone());
    }
    if node == Cave::End {
        println!("{:?}", path);
        return 1;
    }
    for adjacent in &nodes[&node] {
        if small_visited.contains(adjacent) || adjacent == &Cave::Start {
            continue;
        }
        paths += visit_adjacent(nodes, small_visited.clone(), adjacent.clone(), path.clone());
    }
    paths
}

fn parse_line(l: &str) -> (Cave, Cave) {
    let (l, r) = l.split_once("-").unwrap();
    let l = match l.trim() {
        "start" => Cave::Start,
        "end" => Cave::End,
        low if low.chars().nth(0).unwrap().is_lowercase() => Cave::Small(low.to_owned()),
        up if !up.chars().nth(0).unwrap().is_lowercase() => Cave::Big(up.to_owned()),
        _ => unreachable!(),
    };
    let r = match r.trim() {
        "start" => Cave::Start,
        "end" => Cave::End,
        low if low.chars().nth(0).unwrap().is_lowercase() => Cave::Small(low.to_owned()),
        up if !up.chars().nth(0).unwrap().is_lowercase() => Cave::Big(up.to_owned()),
        _ => unreachable!(),
    };
    (l, r)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    Big(String),
    Small(String),
    End,
}
