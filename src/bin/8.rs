fn main() {
    let input = advent_of_code_2021::input("8");

    let count = input
        .lines()
        .flat_map(|l| parse_line(l).unwrap().1)
        .filter(|digit| matches!(digit.len(), 2 | 4 | 3 | 7))
        .count();
    println!("count: {count}");
}

fn parse_line(line: &str) -> Option<([&str; 10], [&str; 4])> {
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
    Some((left, right))
}
