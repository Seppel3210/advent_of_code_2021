fn main() {
    let input = advent_of_code_2021::input("6");
    let days = 256;
    // the count of fish that were born at day i
    let mut fish_amount = vec![0; days + 9];
    for days_until_birth in input.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        fish_amount[days_until_birth] += 1;
    }
    for i in 0..fish_amount.len() {
        let amount = fish_amount[i];
        for day_born in (i + 9..fish_amount.len()).step_by(7) {
            fish_amount[day_born] += amount;
        }
    }

    //println!("{:?}", fish_amount);
    let sum: usize = fish_amount.iter().sum();
    println!("sum: {sum}");
}
