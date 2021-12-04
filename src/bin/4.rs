use std::collections::HashMap;

fn main() {
    let input = advent_of_code_2021::input("4");
    let (numbers, mut cards) = parse_input(&input).unwrap();
    let mut index = HashMap::<_, Vec<_>>::new();
    for (card_id, card) in cards.iter().enumerate() {
        for (y, row) in card.numbers.iter().enumerate() {
            for (x, &num) in row.iter().enumerate() {
                index
                    .entry(num.value)
                    .or_default()
                    .push(NumPosition { card_id, x, y });
            }
        }
    }

    let mut winning_card_id = None;
    let mut winning_num = 0;
    'outer: for number in numbers {
        for &num_to_cross in &index[&number] {
            let NumPosition { card_id, x, y } = num_to_cross;
            if cards[card_id].check_bingo(x, y) {
                winning_card_id = Some(card_id);
                winning_num = number;
                break 'outer;
            }
        }
    }

    for row in &cards[winning_card_id.unwrap()].numbers {
        for num in row {
            print!("|{} {:>2}", if num.crossed { 'x' } else { ' ' }, num.value);
        }
        println!("|");
    }

    let value: u32 = cards[winning_card_id.unwrap()]
        .numbers
        .iter()
        .flat_map(|row| row)
        .filter(|num| !num.crossed)
        .map(|num| num.value as u32)
        .sum();
    println!("solution: {}", winning_num as u32 * value);
}

#[derive(Debug)]
struct Card {
    numbers: Vec<Vec<Num>>,
}

impl Card {
    fn check_bingo(&mut self, x: usize, y: usize) -> bool {
        let nums = &mut self.numbers;
        nums[y][x].crossed = true;
        (0..5).all(|x| nums[y][x].crossed)
        || (0..5).all(|y| nums[y][x].crossed)
        // check for diagonals
        || (x == y) && (0..5).all(|x| nums[x][x].crossed)
        || (4 - x == y) && (0..5).all(|x| nums[4 - x][x].crossed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Num {
    crossed: bool,
    value: u8,
}

#[derive(Clone, Copy)]
struct NumPosition {
    card_id: usize,
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> Option<(Vec<u8>, Vec<Card>)> {
    let mut paragraphs = input.split("\n\n");
    let numbers = paragraphs
        .next()?
        .split(',')
        .map(|x| x.parse().ok())
        .collect::<Option<_>>()?;
    let cards = paragraphs
        .map(|p| {
            let numbers = p
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| {
                            let value = x.parse().ok()?;
                            Some(Num {
                                crossed: false,
                                value,
                            })
                        })
                        .collect()
                })
                .collect::<Option<_>>()?;
            Some(Card { numbers })
        })
        .collect::<Option<_>>()?;
    Some((numbers, cards))
}
