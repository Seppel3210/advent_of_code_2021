fn main() {
    let input = advent_of_code_2021::input("13");
    let mut lines = input.lines();
    let points: Vec<(usize, usize)> = lines
        .by_ref()
        .take_while(|&l| l != "")
        .map(|l| l.split_once(',').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let &(max_x, _) = points.iter().max_by_key(|p| p.0).unwrap();
    let &(_, max_y) = points.iter().max_by_key(|p| p.1).unwrap();
    let mut width = max_x + 1;
    let mut height = max_y + 1;
    let mut paper = vec![false; width * height];

    let folds = lines
        .map(|l| {
            l.strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap()
        })
        .map(|l| match l {
            ("x", x) => (x.parse().unwrap(), 0),
            ("y", y) => (0, y.parse().unwrap()),
            _ => unreachable!(),
        });

    for (x, y) in points {
        paper[x + y * width] = true;
    }
    for (x_0, y_0) in folds.take(1) {
        for x in x_0..width {
            for y in y_0..height {
                let (x_1, y_1) = transform(x_0 as _, y_0 as _, x as _, y as _);
                if [x_1, y_1].iter().any(|&x| x < 0) {
                    continue;
                }
                paper[(x_1 + y_1 * width as i32) as usize] |= paper[x + y * width];
                paper[x + y * width] = false;
            }
        }
        if x_0 != 0 {
            width = x_0;
        }
        if y_0 != 0 {
            height = y_0;
        }
    }

    let count: u32 = paper.iter().map(|&b| b as u32).sum();

    for y in 0..height {
        for x in 0..width {
            if paper[x + y * width] {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }

    println!("{count}");
}

fn transform(x_0: i32, y_0: i32, x: i32, y: i32) -> (i32, i32) {
    match (x_0, y_0) {
        (0, _) => (x, 2 * y_0 - y),
        (_, 0) => (2 * x_0 - x, y),
        _ => unreachable!(),
    }
}
