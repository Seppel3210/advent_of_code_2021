#![feature(array_windows)]

fn main() {
    let input = advent_of_code_2021::input("1");
    let depths: Vec<_> = input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect();
    let count: u32 = depths
        .array_windows()
        .map(|[left, right]| if right > left { 1 } else { 0 })
        .sum();
    println!("{count}");
}
