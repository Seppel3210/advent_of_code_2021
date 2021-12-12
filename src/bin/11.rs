fn main() {
    let input = advent_of_code_2021::input("11");
    let mut levels: Vec<Vec<(u8, bool)>> = input
        .lines()
        .map(|l| {
            (0..l.len())
                .map(|i| (l[i..=i].parse().unwrap(), false))
                .collect()
        })
        .collect();

    let mut flashes = 0;
    for i in 0..100 {
        print!("step: {}, ", i + 1);
        flashes += step(&mut levels);
    }
    println!("{flashes}");
}

fn step(levels: &mut Vec<Vec<(u8, bool)>>) -> usize {
    let mut flashes = 0;
    for y in 0..levels.len() {
        for x in 0..levels[0].len() {
            flashes += increment(levels, x as isize, y as isize)
        }
    }

    for (_, flashed) in levels.iter_mut().flat_map(|row| row.iter_mut()) {
        *flashed = false;
    }

    println!("flashes: {flashes}");
    for row in levels {
        for (level, _) in row {
            print!("{level}");
        }
        println!();
    }
    println!();

    flashes
}

fn increment(levels: &mut Vec<Vec<(u8, bool)>>, x: isize, y: isize) -> usize {
    match levels
        .get_mut(y as usize)
        .and_then(|row| row.get_mut(x as usize))
    {
        Some((level, flashed)) if !*flashed => {
            *level += 1;
            let mut flashes = 0;
            if *level > 9 {
                flashes += 1;
                *flashed = true;
                *level = 0;
                for (dx, dy) in [-1, 0, 1]
                    .into_iter()
                    .flat_map(|x| [(x, -1), (x, 0), (x, 1)])
                {
                    flashes += increment(levels, x + dx, y + dy);
                }
            }
            flashes
        }
        _ => 0,
    }
}
