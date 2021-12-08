// 0: abc efg 6
// 1:   c  f  2
// 2: a cde g 5
// 3: a cd fg 5
// 4:  bcd f  4
// 5: ab d fg 5
// 6: ab defg 6
// 7: a c  f  3
// 8: abcdefg 7
// 9: abcd fg 6
//
// #2:
// 1:   c  f
//
// #3:
// 7: a c  f
//
// #4:
// 4:  bcd f
//
// #5:
// 2: a cde g
// 3: a cd fg
// 5: ab d fg
//
// #6:
// 0: abc efg
// 6: ab defg
// 9: abcd fg
//
// #7:
// 8: abcdefg
//
// a = #3 - #2
// g = #5 - #4 - a
// e = #7 - #4 - a - g
// f = #6 ∩ #2
// c = #2 - f
// bd = #4 - c - f
// b = #6 ∩ bd
// d = bd - b

use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let input = advent_of_code_2021::input("8");

    let digits: HashMap<BTreeSet<_>, _> = HashMap::from([
        ("abcefg".chars().collect(), 0),
        ("cf".chars().collect(), 1),
        ("acdeg".chars().collect(), 2),
        ("acdfg".chars().collect(), 3),
        ("bcdf".chars().collect(), 4),
        ("abdfg".chars().collect(), 5),
        ("abdefg".chars().collect(), 6),
        ("acf".chars().collect(), 7),
        ("abcdefg".chars().collect(), 8),
        ("abcdfg".chars().collect(), 9),
    ]);

    let mut sum = 0;
    for (samples, output) in input.lines().map(|l| parse_line(l).unwrap()) {
        // samples grouped by lenght
        let mut len_map: HashMap<_, Vec<_>> = HashMap::new();
        for sample in samples {
            len_map.entry(sample.len()).or_default().push(sample);
        }
        let map = solve(&len_map);
        let value = output
            .into_iter()
            .map(|chars| {
                let digit = chars.into_iter().map(|c| map[&c]).collect();
                digits[&digit]
            })
            .reduce(|acc, x| acc * 10 + x)
            .unwrap();
        sum += value;
    }
    println!("{sum}");
}

fn solve(samples: &HashMap<usize, Vec<HashSet<char>>>) -> HashMap<char, char> {
    let diff = HashSet::difference;
    let intersect = HashSet::intersection;
    let aref = std::array::from_ref;
    let a = set_op(&samples[&3], &samples[&2], diff);
    let g = set_op(
        aref(&set_op(&samples[&5], &samples[&4], diff)),
        aref(&a),
        diff,
    );
    let e = set_op(
        aref(&set_op(
            aref(&set_op(&samples[&7], &samples[&4], diff)),
            aref(&a),
            diff,
        )),
        aref(&g),
        diff,
    );
    let f = set_op(&samples[&6], &samples[&2], intersect);
    let c = set_op(&samples[&2], aref(&f), diff);
    let bd = set_op(aref(&set_op(&samples[&4], aref(&c), diff)), aref(&f), diff);
    let b = set_op(&samples[&6], aref(&bd), intersect);
    let d = set_op(aref(&bd), aref(&b), diff);
    HashMap::from(
        [
            (a, 'a'),
            (b, 'b'),
            (c, 'c'),
            (d, 'd'),
            (e, 'e'),
            (f, 'f'),
            (g, 'g'),
        ]
        .map(|(s, c)| (s.into_iter().next().unwrap(), c)),
    )
}

fn set_op<'a, F, H>(a: &'a [HashSet<char>], b: &'a [HashSet<char>], op: F) -> HashSet<char>
where
    F: Fn(&'a HashSet<char>, &'a HashSet<char>) -> H,
    H: IntoIterator<Item = &'a char>,
{
    assert!(b.len() == 1);
    let b = &b[0];

    a.iter()
        .map(|a| op(a, b).into_iter().copied().collect::<HashSet<_>>())
        .min_by_key(|set| set.len())
        .unwrap()
}

fn parse_line(line: &str) -> Option<([HashSet<char>; 10], [HashSet<char>; 4])> {
    let (left, right) = line.split_once('|')?;
    let l_words = left.split_whitespace();
    let mut left = [""; 10];
    for (i, word) in l_words.enumerate() {
        left[i] = word;
    }
    let r_words = right.split_whitespace();
    let mut right = [""; 4];
    for (i, word) in r_words.enumerate() {
        right[i] = word;
    }

    let to_hash_set = |s: &str| s.chars().collect();
    Some((left.map(to_hash_set), right.map(to_hash_set)))
}
