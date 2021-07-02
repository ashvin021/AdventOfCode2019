use itertools::iterate;

use aoc2019::*;

fn part01(input: &[u32]) -> u32 {
    input.iter().map(|x| x / 3 - 2).filter(|&x| x > 0).sum()
}

fn part02(input: &[u32]) -> u32 {
    let calculate_fuel = |x: &i32| *x / 3 - 2;
    input
        .iter()
        .map(|&m| {
            let fuel = calculate_fuel(&(m as i32));
            iterate(fuel, calculate_fuel)
                .take_while(|&f| f > 0)
                .sum::<i32>() as u32
        })
        .sum()
}

fn day_01() -> (u32, u32) {
    let input: Vec<String> = get_input(1).collect();
    let nums: Vec<u32> = input.iter().map(|l| l.parse::<u32>().unwrap()).collect();
    (part01(&nums), part02(&nums))
}

timed_main!(100, day_01());
