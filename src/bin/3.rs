fn main() {
    let input = advent_of_code_2021::input("3");

    let len = input.lines().next().unwrap().len();
    let mut counts = vec![0; len];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => counts[i] += 1,
                '0' => counts[i] -= 1,
                _ => panic!("invalid character: {c}"),
            }
        }
    }
    let gamma: u32 = counts
        .iter()
        .map(|&x| if x < 0 { 0 } else { 1 })
        .reduce(|acc, x| acc << 1 | x)
        .unwrap();
    let epsilon = !gamma & (1 << len) - 1;
    println!("{}", epsilon * gamma);
}
