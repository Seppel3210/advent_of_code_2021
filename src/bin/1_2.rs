#![feature(array_windows)]

fn main() {
    let input = advent_of_code_2021::input("1");
    let depths: Vec<u32> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    let depth_sums: Vec<u32> = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    let count: u32 = depth_sums
        .array_windows()
        .map(|[left, right]| if right > left { 1 } else { 0 })
        .sum();
    println!("{count}");
}
