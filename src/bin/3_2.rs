fn main() {
    let input = advent_of_code_2021::input("3");

    let len = input.lines().next().unwrap().len();
    let mut oxy_nums: Vec<_> = input
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();
    let mut co2_nums = oxy_nums.clone();

    for i in (0..len).rev() {
        if oxy_nums.len() == 1 {
            break;
        }
        let most_common = find_most_common_value(&oxy_nums, i as u32);
        oxy_nums.retain(|x| x & 1 << i == most_common << i);
    }

    for i in (0..len).rev() {
        if co2_nums.len() == 1 {
            break;
        }
        let most_common = find_most_common_value(&co2_nums, i as u32);
        co2_nums.retain(|x| x & 1 << i != most_common << i);
    }
    println!("{}", oxy_nums[0] * co2_nums[0]);
}

fn find_most_common_value(nums: &[u32], bit_index: u32) -> u32 {
    let mut count = 0;
    for num in nums {
        if num & 1 << bit_index != 0 {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count < 0 {
        0
    } else {
        1
    }
}
