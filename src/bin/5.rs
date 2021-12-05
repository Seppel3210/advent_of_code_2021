use std::collections::HashMap;

fn main() {
    let input = advent_of_code_2021::input("5");
    let lines = input
        .lines()
        .map(|l| Line::parse(l).unwrap())
        // only use vertical and horizontal lines
        .filter(|l| l.x1 == l.x2 || l.y1 == l.y2);
    let mut map = HashMap::new();
    let mut collisions = 0;
    for line in lines {
        collisions += draw_line(&mut map, line);
    }

    println!("{collisions}");
}

struct Line {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
}

impl Line {
    fn to_points(&self) -> impl Iterator<Item = (i16, i16)> + '_ {
        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;

        let len = dx.abs().max(dy.abs());
        let dx = dx.signum();
        let dy = dy.signum();
        (0..=len).map(move |t| (self.x1 + t * dx, self.y1 + t * dy))
    }

    fn parse(input: &str) -> Option<Self> {
        let (point1, point2) = input.split_once("->")?;
        let (x1, y1) = point1.split_once(",")?;
        let (x2, y2) = point2.split_once(",")?;
        Some(Line {
            x1: x1.trim().parse().ok()?,
            y1: y1.trim().parse().ok()?,
            x2: x2.trim().parse().ok()?,
            y2: y2.trim().parse().ok()?,
        })
    }
}

fn draw_line(map: &mut HashMap<(i16, i16), u16>, line: Line) -> u32 {
    let mut collisions = 0;
    for point in line.to_points() {
        let count = map.entry(point).or_default();
        if *count == 1 {
            collisions += 1;
        }

        *count += 1;
    }
    collisions
}
