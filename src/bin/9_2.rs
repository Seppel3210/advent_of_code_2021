use std::collections::BinaryHeap;

fn main() {
    let input = advent_of_code_2021::input("9");
    let mut depths: Vec<Vec<(u8, bool)>> = input
        .lines()
        .map(|l| {
            (0..l.len())
                .map(|i| (l[i..=i].parse().unwrap(), false))
                .collect()
        })
        .collect();

    let mut largest_basins = BinaryHeap::new();
    for y in 0..depths.len() {
        for x in 0..depths[0].len() {
            let (depth, visited) = depths[y][x];
            if visited || depth >= 9 {
                continue;
            }
            largest_basins.push(size_of_basin(&mut depths, x as isize, y as isize));
        }
    }
    let mut answer = 1;
    for _ in 0..3 {
        answer *= largest_basins.pop().unwrap();
    }
    println!("{answer}");
}

fn size_of_basin(depths: &mut Vec<Vec<(u8, bool)>>, x: isize, y: isize) -> u32 {
    match depths
        .get_mut(y as usize)
        .and_then(|col| col.get_mut(x as usize))
    {
        Some((depth, visited)) if !*visited && *depth < 9 => {
            *visited = true;
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(|(dx, dy)| size_of_basin(depths, x + dx, y + dy))
                .sum::<u32>()
                + 1
        }
        _ => 0,
    }
}
