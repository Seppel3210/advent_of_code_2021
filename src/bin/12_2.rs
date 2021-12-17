use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
};

fn main() {
    let input = advent_of_code_2021::input("12");
    let mut nodes = BTreeMap::new();
    for (a, b) in input.lines().map(parse_line) {
        nodes.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
        nodes.entry(b).or_insert(Vec::new()).push(a);
    }
    let paths = visit_adjacent(
        &nodes,
        HashSet::new(),
        Cave::Start,
        /*Vec::new(),*/ false,
    );
    println!("{paths}");
}

fn visit_adjacent(
    nodes: &BTreeMap<Cave, Vec<Cave>>,
    mut small_visited: HashSet<Cave>,
    node: Cave,
    //mut path: Vec<Cave>,
    mut visited_twice: bool,
) -> usize {
    let mut paths = 0;
    //path.push(node.clone());
    if node == Cave::End {
        return 1;
    }
    if matches!(node, Cave::Small(_)) {
        visited_twice |= !small_visited.insert(node.clone());
    }
    for adjacent in &nodes[&node] {
        if small_visited.contains(adjacent) && visited_twice || adjacent == &Cave::Start {
            continue;
        }

        paths += visit_adjacent(
            nodes,
            small_visited.clone(),
            adjacent.clone(),
            /*path.clone(),*/
            visited_twice,
        );
    }
    paths
}

fn parse_line(l: &str) -> (Cave, Cave) {
    let (l, r) = l.split_once("-").unwrap();
    let parse_cave = |s: &str| match s.trim() {
        "start" => Cave::Start,
        "end" => Cave::End,
        low if low.chars().nth(0).unwrap().is_lowercase() => Cave::Small(low.to_owned()),
        up if !up.chars().nth(0).unwrap().is_lowercase() => Cave::Big(up.to_owned()),
        _ => unreachable!(),
    };
    (parse_cave(l), parse_cave(r))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cave {
    Start,
    Big(String),
    Small(String),
    End,
}
