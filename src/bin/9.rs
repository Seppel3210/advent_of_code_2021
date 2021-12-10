#![feature(try_blocks)]
#![feature(type_ascription)]

fn main() {
    let input = advent_of_code_2021::input("9");
    let depths: Vec<Vec<u8>> = input
        .lines()
        .map(|l| (0..l.len()).map(|i| l[i..=i].parse().unwrap()).collect())
        .collect();
    let mut sum = 0;
    for y in 0..depths.len() {
        for x in 0..depths[0].len() {
            if [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().all(|&(dx, dy)| {
                (try {
                    depths[y][x]
                        < *depths
                            .get((y as i32 + dy) as usize)?
                            .get((x as i32 + dx) as usize)?
                }: Option<_>)
                    .unwrap_or(true)
            }) {
                sum += (depths[y][x] + 1) as u32;
            }
        }
    }

    println!("{sum}");
}
