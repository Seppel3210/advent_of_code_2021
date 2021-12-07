fn main() {
    let input = advent_of_code_2021::input("7");

    let mut v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();


    let min_cost = (0..*v.iter().max().unwrap()).map(|x| {
        (x, v.iter()
            .map(|c| {
                let x = (x - c).abs();
                (x * x + x) / 2
            })
            .sum::<i64>())
    }).min_by_key(|(_, x)| *x).unwrap();
    println!("cost: {min_cost:?}");
    let len = v.len();
    let (_, &mut mean, _) = v.select_nth_unstable(len / 2);

    println!("{mean}");
}
