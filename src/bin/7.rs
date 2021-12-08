#![feature(int_abs_diff)]

fn main() {
    let input = advent_of_code_2021::input("7");

    let mut v: Vec<u32> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let len = v.len();
    let (_, &mut median, _) = v.select_nth_unstable(len / 2);
    let cost: u32 = v.iter().map(|x| x.abs_diff(median)).sum();
    println!("{cost}");
}
