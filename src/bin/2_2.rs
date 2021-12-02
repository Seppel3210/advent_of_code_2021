fn main() {
    let input = advent_of_code_2021::input("2");
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for dir in input.lines().map(|l| parse_line(l).unwrap()) {
        match dir {
            Dir::Forward(d) => {
                x += d;
                y += d * aim;
            }
            Dir::Down(d) => aim += d,
            Dir::Up(d) => aim -= d,
        }
    }
    println!("{}", x * y);
}

enum Dir {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_line(line: &str) -> Option<Dir> {
    let (dir, amount) = line.split_once(" ")?;
    let dir = match dir {
        "forward" => Dir::Forward,
        "down" => Dir::Down,
        "up" => Dir::Up,
        _ => return None,
    };
    Some(dir(amount.trim().parse().ok()?))
}
